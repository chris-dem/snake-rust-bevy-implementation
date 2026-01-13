/**
 *
 * Ideally the following API should be optimised such that each player has its own optimised output
 * Can be refactored later down the line so we will denote this as a TODO task
 */
use crate::prelude::{api::*, common::*, snake::*};
use anyhow::Result as ARes;
use rand::prelude::*;
use rayon::prelude::*;

pub trait PlayerTrait {
    fn choose_dir(&mut self, game_instance: &GameAPI, with_rng: &mut dyn RngCore) -> Direction;
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
    Step,
    Food,
    Won,
    Lost,
}

impl Simulator {
    pub fn new(game_builder: GameAPIBuilder, simulator_options: SimulatorOptions) -> Self {
        Self {
            game_builder, simulator_options,
        }
    }
    pub fn simulation(
        &self,
        player: &mut impl PlayerTrait,
        rng: &mut dyn RngCore,
    ) -> ARes<Vec<SimulationStep>> {
        let mut game_instance = self.game_builder.build(Some(rng));
        let mut snapshots = vec![];
        loop {
            let before_step = game_instance.num_of_apples;
            let before_step_repr = game_instance.to_game_repr();
            let dir = player.choose_dir(&game_instance, rng);
            game_instance.snake.direction = dir;
            let mut num_iter = 0usize;
            let next_step = game_instance.next(rng)?;
            let otp = match next_step {
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
                    num_iter += 1;
                    SimulationStep {
                        snapshot: before_step_repr,
                        direction: dir,
                        reward: if before_step == game_instance.num_of_apples {
                            SimulationStepReward::Step
                        } else {
                            SimulationStepReward::Food
                        },
                        next_state: Some(game_instance.to_game_repr()),
                    }
                }
            };
            snapshots.push(otp);
            if matches!(next_step, StepResult::Win { .. } | StepResult::Lost { .. }) {
                break;
            } else if num_iter == self.simulator_options.number_of_iterations {
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
