/**
 *
 * Ideally the following API should be optimised such that each player has its own optimised output
 * Can be refactored later down the line so we will denote this as a TODO task
 */
use crate::prelude::{api::*, common::*, snake::*};
use anyhow::Result as ARes;
use itertools::Itertools;
use rand::prelude::*;
use strum::IntoEnumIterator;

pub trait PlayerTrait {
    fn choose_dir(&self, game_instance: &GameAPI, with_rng: &mut dyn RngCore) -> Direction;
}

#[derive(Debug, Clone, Copy)]
pub struct Simulator {
    game_builder: GameAPIBuilder,
    pub simulator_options: SimulatorOptions,
}

#[derive(Debug, Clone, Copy)]
pub struct SimulatorOptions {
    pub number_of_iterations: usize,
}

#[derive(Clone, Debug)]
pub struct SimulationStep {
    pub snapshot: GameAPIBinaryRepr,
    pub direction: Direction,
    pub reward: SimulationStepReward,
    pub next_state: Option<GameAPIBinaryRepr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimulationStepReward {
    Step(bool),
    Food,
    Won,
    Lost,
}

impl Simulator {
    pub fn new(game_builder: GameAPIBuilder, simulator_options: SimulatorOptions) -> Self {
        Self {
            game_builder,
            simulator_options,
        }
    }

    fn handle_next_step(
        &self,
        before_step: u128,
        before_step_repr: GameAPIBinaryRepr,
        dir: Direction,
        next_step: StepResult,
        game_instance: &GameAPI,
    ) -> SimulationStep {
        match next_step {
            StepResult::Lost { .. } => SimulationStep {
                snapshot: before_step_repr,
                direction: dir,
                reward: SimulationStepReward::Lost,
                next_state: None,
            },
            StepResult::Win { .. } => SimulationStep {
                snapshot: before_step_repr,
                direction: dir,
                reward: SimulationStepReward::Won,
                next_state: None,
            },
            StepResult::Base => {
                let step_rew = if game_instance.snake.size >= 10 {
                    false
                } else {
                    let sn_head = game_instance.snake.head;
                    let ap_head = game_instance.apples;
                    let s = Direction::iter().collect_array::<4>().unwrap().map(|d| {
                        let t1 = game_instance
                            .snake
                            .check_cell(sn_head.add_dir(d))
                            .is_some_and(|x| !x);
                        t1 && sn_head.l1(ap_head) > sn_head.add_dir(d).l1(ap_head)
                    });
                    s[dir as usize]
                };
                // let measure_optimal = _;
                SimulationStep {
                    snapshot: before_step_repr,
                    direction: dir,
                    reward: if before_step == game_instance.num_of_apples {
                        SimulationStepReward::Step(step_rew)
                    } else {
                        SimulationStepReward::Food
                    },
                    next_state: Some(game_instance.to_game_repr()),
                }
            }
        }
    }

    pub fn simulation(
        &self,
        player: &impl PlayerTrait,
        rng: &mut impl RngCore,
        with_summary: bool,
    ) -> ARes<Vec<SimulationStep>> {
        let mut game_instance = self.game_builder.build(Some(rng));
        let mut snapshots = vec![];
        let mut num_iter = 0usize;
        loop {
            num_iter += 1;
            let before_step = game_instance.num_of_apples;
            let before_step_repr = game_instance.to_game_repr();
            let dir = player.choose_dir(&game_instance, rng);
            game_instance.update_direction(dir);
            let next_step = game_instance.next(rng)?;
            let otp = self.handle_next_step(
                before_step,
                before_step_repr,
                dir,
                next_step,
                &game_instance,
            );
            snapshots.push(otp);
            if matches!(next_step, StepResult::Win { .. } | StepResult::Lost { .. }) {
                if with_summary {
                    dbg!((game_instance.num_of_apples, game_instance.steps));
                }
                break;
            } else if num_iter >= self.simulator_options.number_of_iterations {
                snapshots.push(SimulationStep {
                    snapshot: game_instance.to_game_repr(),
                    direction: dir,
                    reward: SimulationStepReward::Lost,
                    next_state: None,
                });
                break;
            }
        }
        ARes::Ok(snapshots)
    }
}
