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
    simulator_options: SimulatorOptions,
}

#[derive(Debug, Clone, Copy)]
pub struct SimulatorOptions {
    number_of_iterations: usize,
}

pub struct SimulationStep {
    pub snapshot: GameAPIBinaryRepr,
    pub direction: Direction,
    pub reward: i32,
    pub next_state: Option<(GameAPIBinaryRepr, Direction)>,
}

impl Simulator {
    pub fn simulation(
        &self,
        mut player: impl PlayerTrait,
        rng: &mut dyn RngCore,
    ) -> ARes<Vec<SimulationStep>> {
        let mut game_instance = self.game_builder.build(Some(rng));
        let mut snapshots = vec![];
        loop {
            let before_step = game_instance.num_of_apples;
            let dir = player.choose_dir(&game_instance, rng);
            game_instance.snake.direction = dir;
            let mut num_iter = 0usize;
            let next_step = game_instance.next(rng)?;
            let otp = match next_step {
                StepResult::Lost { .. } => SimulationStep {
                    snapshot: todo!("Get binarised game instance"),
                    direction: dir,
                    reward: -100,
                    next_state: None,
                },
                StepResult::Win { .. } => SimulationStep {
                    snapshot: todo!("Get binarised game instance"),
                    direction: dir,
                    reward: 100,
                    next_state: None,
                },
                StepResult::Base => {
                    num_iter += 1;
                    let dir_next = player.choose_dir(&game_instance, rng);
                    SimulationStep {
                        snapshot: todo!("Get binarised game instance"),
                        direction: dir,
                        reward: if before_step == game_instance.num_of_apples {
                            -1
                        } else {
                            10
                        },
                        next_state: Some(todo!("Game instaec")),
                    }
                }
            };
            snapshots.push(otp);
            if matches!(
                next_step,
                (StepResult::Win { .. } | StepResult::Lost { .. })
            ) {
                break;
            } else if num_iter == self.simulator_options.number_of_iterations {
                snapshots.push(SimulationStep {
                    snapshot: todo!("Get binarised game instance"),
                    direction: dir,
                    reward: -100,
                    next_state: None,
                });
                break;
            }
        }
        ARes::Ok(snapshots)
    }

    pub fn par_simulation<
        RngType: RngCore + Clone + SeedableRng + Send + Sync,
        PlayerType: PlayerTrait + Clone + Send + Sync,
    >(
        &self,
        num_sims: usize,
        player: PlayerType,
        rng: &mut RngType,
    ) -> ARes<Vec<SimulationStep>> {
        let res: Vec<_> = (0..num_sims)
            .map(|i| rng.next_u64().wrapping_add(i as u64))
            .par_bridge()
            .map(|seed| {
                let mut rng = RngType::seed_from_u64(seed);
                self.simulation(player.clone(), &mut rng)
            })
            .collect::<ARes<_>>()?;
        ARes::Ok(res.into_iter().flatten().collect::<Vec<_>>())
    }
}
