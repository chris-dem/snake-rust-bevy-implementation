use std::sync::{Arc, OnceLock, RwLock};

use burn::{
    data::{
        dataloader::{DataLoader, batcher::*},
        dataset::vision::MnistItem,
    },
    prelude::*,
    tensor::ops::BoolTensor,
};
use itertools::Itertools;
use rand::prelude::*;
use snake_api_lib::{
    api::{GameAPI, GameAPIBinaryRepr},
    common::{Direction, GRID_X, GRID_Y},
    simulator::{PlayerTrait, Simulator},
};
use strum::IntoEnumIterator;

use crate::model::{Model, StateRepr};

#[derive(Debug)]
pub(crate) struct DatasetGenerator {
    gen_config: DatasetGeneratorConfig,
    simulator: Simulator,
    cached_runs: Vec<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct DatasetGeneratorConfig {
    pub number_eps: usize,
    pub ep_limit: usize,
    pub batch_size: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct PlayerModel<B: Backend> {
    model: Arc<Model<B>>,
    eps: f64,
    device: Arc<B::Device>,
    active_mode: bool,
}

unsafe impl<B: Backend> Send for PlayerModel<B> {}

impl<B: Backend> PlayerModel<B> {
    fn update_model<'c>(self, new_model: Arc<Box<Model<B>>>) -> PlayerModel<B> {
        PlayerModel {
            model: new_model,
            device: self.device,
            eps: self.eps,
            active_mode: true,
        }
    }

    fn set_mode(&mut self, is_training: bool) {
        self.active_mode = is_training;
    }

    fn set_eps(&mut self, eps: f64) {
        self.eps = eps;
    }
}

impl<B: Backend> PlayerTrait for PlayerModel<B> {
    fn choose_dir(&mut self, game_instance: &GameAPI, with_rng: &mut dyn RngCore) -> Direction {
        let dir = game_instance.snake.direction;
        let mut dir_vec = Direction::iter().enumerate().collect_vec();
        dir_vec.remove(dir.inverse() as usize);
        if with_rng.random_bool(self.eps) && self.active_mode {
            return dir_vec.choose(with_rng).unwrap().1;
        }
        let GameAPIBinaryRepr(arr) = game_instance.to_game_repr();
        let arr = arr
            .to_shape((1, GRID_X, GRID_Y))
            .expect("Padding with one should nof affect it");
        let arr = arr
            .as_standard_layout()
            .to_owned()
            .map(|x| *x as i32)
            .into_raw_vec_and_offset()
            .0;
        let td = TensorData::new(arr, [1, GRID_X, GRID_Y]);
        let td: Tensor<B, 4, Float> = Tensor::<B, 3, Int>::from_data(td, self.device.as_ref())
            .one_hot(4)
            .float();
        let state_repr: StateRepr<B> = StateRepr(td);
        let m = Tensor::<B, 1, Int>::from_data(
            dir_vec
                .into_iter()
                .map(|x| x.0 as i32)
                .collect_array::<3>()
                .unwrap(),
            self.device.as_ref(),
        );
        let out: Tensor<B, 1> = self
            .model
            .forward(state_repr)
            .flatten(0, 1)
            .select(0, m);
        let indx = out.argmax(0).into_scalar().elem::<i64>();
        let indx = indx as usize;
        Direction::from(indx)
    }
}

impl DatasetGenerator {
    fn new(gen_config: DatasetGeneratorConfig, simulator: Simulator) -> Self {
        Self {
            gen_config,
            simulator,
            cached_runs: Vec::new(),
        }
    }

    fn run_sims<T, B: Backend>(&mut self, player: PlayerModel<B>, with_rng: &mut T)
    where
        T: RngCore + Clone + SeedableRng + Send + Sync,
    {
        let p = self.gen_config.number_eps;
        let mut res = self.simulator.par_simulation(p, player, with_rng).expect("No errors");
        res.shuffle(with_rng);
        self.cached_runs = res;
    }
}
