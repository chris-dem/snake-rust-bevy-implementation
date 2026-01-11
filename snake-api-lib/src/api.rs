use std::fmt::{Debug, Display};

use crate::{
    common::{Cell, Coord, GRID_X, GRID_Y},
    snake::{ArrSnake, SnakeTrait},
};
use anyhow::Result as ARes;
use ndarray::prelude::*;
use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepResult {
    Win {
        num_steps: usize,
    },
    Lost {
        num_steps: usize,
        number_of_fruits: usize,
        snake_size: usize,
        level_reached: Speed,
    },
    Base,
}

#[derive(Debug, Clone, Copy)]
pub struct GameAPI {
    pub snake: ArrSnake,
    pub apples: Coord,
    pub steps: u128,
    pub num_of_apples: u128,
    pub score: u128,
    pub mode: Speed,
    pub game_options: GameOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GameAPIBuilder {
    selected_game_options: Option<GameOptions>,
}

impl GameAPIBuilder {
    pub fn build(self, rng: Option<&mut dyn RngCore>) -> GameAPI {
        GameAPI::new(rng, self.selected_game_options)
    }

    pub fn with_selected_game_options(mut self, selected_game_options: GameOptions) -> Self {
        self.selected_game_options = Some(selected_game_options);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Speed {
    #[default]
    Slow,
    Medium,
    Hard,
    VeryHard,
    GodMode,
}

#[derive(Debug, Clone, Default)]
pub struct GameAPIBinaryRepr {
    pub board: Array2<bool>, // X Y [Empty, Food, Snake, Head]
    pub direction: Array1<bool>,
}

impl Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Speed::Slow => "Easy",
            Speed::Medium => "Medium",
            Speed::Hard => "Hard",
            Speed::VeryHard => "Very Hard",
            Speed::GodMode => "GodLike",
        };
        write!(f, "{s}")
    }
}

impl Speed {
    fn to_score(self) -> u128 {
        match self {
            Self::Slow => 10,
            Self::Medium => 20,
            Self::Hard => 50,
            Self::VeryHard => 100,
            Self::GodMode => 150,
        }
    }

    pub fn to_time_speed(self) -> f32 {
        match self {
            Self::Slow => 1.,
            Self::Medium => 0.9,
            Self::Hard => 0.8,
            Self::VeryHard => 0.7,
            Self::GodMode => 0.5,
        }
    }
}

impl Display for GameAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for i in 0..=GRID_Y {
            write!(f, "{:^3}|", i)?;
        }
        writeln!(f)?;
        for i in 0..GRID_X {
            let i_print = i + 1;
            write!(f, "{i_print:^3}|")?;
            for j in 0..GRID_Y {
                let indx = Coord {
                    row: i as u8,
                    col: j as u8,
                };
                if indx == self.snake.head {
                    write!(f, "{:^3}|", 'H')?;
                } else if indx == self.snake.tail {
                    write!(f, "{:^3}|", 'T')?;
                } else if indx == self.apples {
                    write!(f, "{:^3}|", 'A')?;
                } else if self.snake.check_cell(indx).is_some_and(|x| x) {
                    write!(f, "{:^3}|", 'S')?;
                } else {
                    write!(f, "{:^3}|", '*')?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameOptions {
    time_speed_del: u128,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            time_speed_del: ((GRID_X * GRID_Y) / 5) as u128,
        }
    }
}

impl GameAPI {
    pub fn new(rng: Option<&mut dyn RngCore>, game_options: Option<GameOptions>) -> Self {
        let rng = match rng {
            None => &mut SmallRng::from_os_rng(),
            Some(rng) => rng,
        };
        let mid = Coord::middle();
        let c = loop {
            let c = Coord {
                row: rng.random_range(0..GRID_X as u8),
                col: rng.random_range(0..GRID_Y as u8),
            };
            if mid.l0(c) > 1 {
                break c;
            }
        };

        Self {
            snake: ArrSnake::default(),
            apples: c,
            steps: 0,
            score: 0,
            num_of_apples: 0,
            mode: Speed::default(),
            game_options: game_options.unwrap_or_default(),
        }
    }

    pub fn get_pos(&self, pos: Coord) -> Option<Cell> {
        if pos.row > GRID_X as u8 && pos.col > GRID_Y as u8 {
            return None;
        }
        if self.apples == pos {
            return Some(Cell::Apple);
        }
        if self.snake.check_cell(pos)? {
            Some(Cell::Snake)
        } else {
            Some(Cell::Empty)
        }
    }

    fn set_speed(&mut self) {
        let f = self.snake.size as f64 / (GRID_X * GRID_Y) as f64;
        self.mode = match f {
            (0.0..0.2) => Speed::Slow,
            (0.2..0.4) => Speed::Medium,
            (0.4..0.6) => Speed::Hard,
            (0.6..0.8) => Speed::VeryHard,
            (0.8..=1.) => Speed::GodMode,
            _ => unreachable!("Should not exceed 1"),
        };
    }

    pub fn next(&mut self, rng: &mut dyn RngCore) -> ARes<StepResult> {
        if !self.snake.is_next_valid() {
            return Ok(StepResult::Lost {
                num_steps: self.steps as usize,
                number_of_fruits: self.num_of_apples as usize,
                snake_size: self.snake.size,
                level_reached: self.mode,
            });
        }
        let head = self.snake.next_step()?;
        let with_food = head == self.apples;
        self.snake.step(with_food)?;
        if with_food {
            if let Some(coord) = self.snake.get_free_spot(rng) {
                self.num_of_apples += 1;
                self.apples = coord;
            } else {
                return Ok(StepResult::Win {
                    num_steps: self.steps as usize,
                });
            }
        }
        self.steps += 1;
        self.set_speed();
        self.score += with_food as u128 * self.mode.to_score();
        if self.steps.is_multiple_of(self.game_options.time_speed_del) {
            self.score = self.score.saturating_sub(1);
        }
        Ok(StepResult::Base)
    }
}
