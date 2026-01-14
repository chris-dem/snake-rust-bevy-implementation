use std::sync::{Arc, OnceLock, RwLock};

use anyhow::Result as ARes;
use burn::{
    data::{
        dataloader::{DataLoader, batcher::*},
        dataset::vision::MnistItem,
    },
    prelude::*,
    tensor::{backend::AutodiffBackend, ops::BoolTensor},
};

use itertools::Itertools;
// use itertools::Itertools;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use snake_api_lib::{
    api::{GameAPI, GameAPIBinaryRepr, GameAPIBuilder},
    common::{Direction, GRID_X, GRID_Y},
    simulator::{PlayerTrait, SimulationStep, SimulationStepReward, Simulator, SimulatorOptions},
};
use strum::IntoEnumIterator;

use crate::model::{Model, StateRepr};

#[derive(Debug)]
pub(crate) struct DatasetGenerator {
    data_gen: DatasetGeneratorConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub(crate) struct DatasetGeneratorConfig {
    pub sim_config: SimulationConfig,
    pub rew_config: RewardConfig,
    pub batch_size: usize,
}

impl DatasetGeneratorConfig {
    pub fn build(self) -> DatasetGenerator {
        DatasetGenerator { data_gen: self }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SimulationConfig {
    pub number_episodes: usize,
    pub episode_limit: Option<usize>,
    pub eps_expl: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RewardConfig {
    pub step_rew: f32,
    pub fruit_rew: f32,
    pub win_rew: f32,
    pub lose_rew: f32,
    pub gamma_factor: f32,
}

#[derive(Clone, Debug)]
pub struct PlayerModel<'a, 'b, B: Backend> {
    pub model: &'a Model<B>,
    pub eps: f64,
    pub device: &'b B::Device,
    pub active_mode: bool,
}

impl<'a, 'b, B: Backend> PlayerModel<'a, 'b, B> {
    fn set_mode(&mut self, is_training: bool) {
        self.active_mode = is_training;
    }

    fn set_eps(&mut self, eps: f64) {
        self.eps = eps;
    }
}

impl<B: Backend> From<(GameAPIBinaryRepr, &B::Device)> for StateRepr<B> {
    fn from(value: (GameAPIBinaryRepr, &B::Device)) -> Self {
        let (GameAPIBinaryRepr(arr), dev) = value;
        let arr = arr
            .to_shape([1, GRID_X, GRID_Y])
            .expect("Padding with one should nof affect it");
        let arr = arr
            .as_standard_layout()
            .to_owned()
            .map(|x| *x as i32)
            .into_raw_vec_and_offset()
            .0;
        let td = TensorData::new(arr, [1, GRID_X, GRID_Y]);
        let td: Tensor<B, 4, Float> = Tensor::<B, 3, Int>::from_data(td, dev).one_hot(4).float();
        StateRepr(td)
    }
}

impl<'a, 'b, B: Backend> PlayerTrait for PlayerModel<'a, 'b, B> {
    fn choose_dir(&self, game_instance: &GameAPI, with_rng: &mut dyn RngCore) -> Direction {
        let dir = game_instance.snake.direction;
        let mut dir_vec = Direction::iter().enumerate().collect_vec();
        dir_vec.remove(dir.inverse() as usize);
        if self.active_mode && with_rng.random_bool(self.eps) {
            // unimplemented!("Eps-greedy not implemented due to dropouts");
            return dir_vec.choose(with_rng).unwrap().1;
        }
        let state_repr: StateRepr<B> = (game_instance.to_game_repr(), self.device).into();
        let m = Tensor::<B, 1, Int>::from_data(
            dir_vec
                .into_iter()
                .map(|x| x.0 as i32)
                .collect_array::<3>()
                .unwrap(),
            self.device,
        );
        let out: Tensor<B, 1> = self.model.forward(state_repr).flatten(0, 1).select(0, m);
        let indx = out.argmax(0).into_scalar().elem::<i64>();
        let indx = indx as usize;
        Direction::from(indx)
    }
}

impl DatasetGenerator {
    pub fn iter_with_model<B: Backend, T>(
        &self,
        model: &Model<B>,
        device: &B::Device,
        with_rng: &mut T,
        active_mode: bool,
    ) -> impl Iterator<Item = BatchedSimulationStep<B>>
    where
        T: RngCore + Clone + SeedableRng + Send + Sync,
    {
        let p = self.data_gen.sim_config.number_episodes;
        let player = PlayerModel {
            model,
            eps: self.data_gen.sim_config.eps_expl,
            device,
            active_mode,
        };
        let sim = Simulator::new(
            GameAPIBuilder::default(),
            SimulatorOptions {
                number_of_iterations: self.data_gen.sim_config.episode_limit.unwrap_or(10_000),
            },
        );

        let mut sims = tqdm::tqdm((0..p).map(|_| sim.simulation(&player, with_rng)))
            .collect::<ARes<Vec<_>>>()
            .expect("Should compile")
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        sims.shuffle(with_rng);
        let batches = sims
            .chunks(self.data_gen.batch_size)
            .map(|c| self.batch_sims(device, c.iter().cloned(), &player))
            .collect_vec();
        batches.into_iter()
    }

    fn batch_sims<B: Backend>(
        &self,
        device: &B::Device,
        els: impl Iterator<Item = SimulationStep>,
        player: &PlayerModel<B>,
    ) -> BatchedSimulationStep<B> {
        let mut v_snapshot: Vec<Tensor<B, 4, Float>> = vec![];
        let mut v_direction = vec![];
        let mut v_reward = vec![];
        let mut v_next_state_qual = vec![];
        for SimulationStep {
            snapshot,
            direction,
            reward,
            next_state,
        } in els
        {
            let (StateRepr(snap)): StateRepr<B> = (snapshot, device).into();
            v_snapshot.push(snap);
            v_direction.push(direction as i32);
            match reward {
                SimulationStepReward::Won => {
                    v_reward.push(self.data_gen.rew_config.fruit_rew);
                    v_next_state_qual.push(self.data_gen.rew_config.win_rew);
                }
                SimulationStepReward::Lost => {
                    v_reward.push(self.data_gen.rew_config.step_rew);
                    v_next_state_qual.push(self.data_gen.rew_config.lose_rew);
                }
                SimulationStepReward::Step => {
                    let st: StateRepr<B> = (
                        next_state.expect("Should have next state in step"),
                        player.device,
                    )
                        .into();
                    v_reward.push(self.data_gen.rew_config.step_rew);
                    let out = player.model.forward(st).argmax(0);
                    let el = out.max().into_scalar().elem::<f32>();
                    v_next_state_qual.push(el);
                }
                SimulationStepReward::Food => {
                    let st: StateRepr<B> = (
                        next_state.expect("Sohuld have next state in step"),
                        player.device,
                    )
                        .into();
                    let out = player.model.forward(st).argmax(0);
                    let el = out.max().into_scalar().elem::<f32>();
                    v_reward.push(self.data_gen.rew_config.fruit_rew);
                    v_next_state_qual.push(el);
                }
            }
        }

        let b_size = v_direction.len();
        assert!(b_size == v_reward.len() && b_size == v_next_state_qual.len());
        let snaps = Tensor::cat(v_snapshot, 0);
        BatchedSimulationStep {
            snapshot: snaps,
            direction: Tensor::from_data(TensorData::new(v_direction, [b_size]), player.device),
            reward: Tensor::from_data(TensorData::new(v_reward, [b_size]), player.device),
            next_state_qual: Tensor::from_data(
                TensorData::new(v_next_state_qual, [b_size]),
                player.device,
            ),
        }
    }
}

pub struct BatchedSimulationStep<B: Backend> {
    pub snapshot: Tensor<B, 4, Float>,
    pub direction: Tensor<B, 1, Int>,
    pub reward: Tensor<B, 1, Float>,
    pub next_state_qual: Tensor<B, 1, Float>,
}
