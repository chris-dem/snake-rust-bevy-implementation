use crate::{
    common::{Cell, Coord, GRID_X, GRID_Y},
    snake::{ArrSnake, SnakeTrait},
};
use anyhow::Result as ARes;
use rand::prelude::*;

pub mod common;
pub mod snake;

pub enum GameState {
    Win,
    Lost,
    Base,
}

#[derive(Debug, Clone, Copy)]
pub struct GameAPI {
    pub snake: ArrSnake,
    pub apples: Coord,
}

impl GameAPI {
    pub fn new(rng: Option<&mut dyn RngCore>) -> Self {
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

    pub fn next(&mut self, rng: &mut dyn RngCore) -> ARes<GameState> {
        if !self.snake.is_next_valid() {
            return Ok(GameState::Lost);
        }
        let head = self.snake.next_step()?;
        let with_food = head == self.apples;
        self.snake.step(with_food)?;
        if let Some(coord) = self.snake.get_free_spot(rng) {
            self.apples = coord;
            Ok(GameState::Base)
        } else {
            Ok(GameState::Win)
        }
    }
}
