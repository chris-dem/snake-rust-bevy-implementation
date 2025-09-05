use rand::prelude::*;

use crate::{common::Coord, snake::{ArrSnake, SnakeTrait}};


pub mod common;
pub mod snake;


#[derive(Debug, Clone, Copy)]
pub struct GameAPI {
    snake: ArrSnake,
    apples: Coord,
}

impl GameAPI {
    fn new(rng: Option<&mut dyn RngCore>) -> Self {
        let rng = match rng {
            None => &mut SmallRng::from_os_rng(),
            Some(rng) => rng,
        };
        let mid = Coord::middle();
        let c = loop {
            let c = Coord {
                i: rng.random_range(0..GRID_X as u8),
                j: rng.random_range(0..GRID_Y as u8),
            };
            if mid.l0(c) > 1 {
                break c;
            }
        };

        Self {
            direction: Direction::Left,
            head: vec![Coord::middle()],
            apples: c,
        }
    }

    fn get_pos(&self, x: u8, y: u8) -> Option<Cell> {
        if x > GRID_X && y > GRID_Y {
            None
        }
    }
}
