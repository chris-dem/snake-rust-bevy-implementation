use crate::common::{Coord, Direction, GRID_X, GRID_Y};
use anyhow::{Result as AResult, anyhow};
use bitvec::prelude::*;

// Fixed-size bit array
type GridBits = BitArr!(for GRID_X * GRID_Y, in u64); // 768 bits using u64 storage

pub trait SnakeTrait {
    fn check_cell(&self, coords: Coord) -> Option<bool>;
    fn direction(&mut self, dir: Direction);
    fn step(&mut self, with_food: bool) -> AResult<()>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ArrSnake {
    maps: [GridBits; 4],
    direction: Direction,
    head: Coord,
    tail: Coord,
}

impl ArrSnake {
    fn insert(&mut self, indx: usize, direction: Direction) -> Option<()> {
        if indx > GRID_X * GRID_Y {
            return None;
        }
        self.maps[direction as usize].set(indx, true);
        Some(())
    }
}

impl SnakeTrait for ArrSnake {
    fn check_cell(&self, coords: Coord) -> Option<bool> {
        let indx = coords.into_index();
        if indx > GRID_X * GRID_Y {
            return None;
        }
        Some(self.maps.iter().all(|arr| arr[indx]))
    }

    fn direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    fn step(&mut self, with_food: bool) -> AResult<()> {
        let c = self.head;
        let res = match (c, self.direction) {
            (Coord { j: 0, .. }, Direction::Left) | (Coord { i: 0, .. }, Direction::Up) => {
                Err(anyhow!("Invalid coordinate {:?}, {:?}", c, self.direction))
            }
            (Coord { i, .. }, Direction::Down) if i == GRID_X as u8 => {
                Err(anyhow!("Invalid coordinate {:?}, {:?}", c, self.direction))
            }
            (Coord { j, .. }, Direction::Right) if j == GRID_X as u8 => {
                Err(anyhow!("Invalid coordinate {:?}, {:?}", c, self.direction))
            }
            (Coord { i, j }, d) => {
                let d = d as i8;
                let (ni, nj) = if d % 2 == 0 {
                    (i, (j as i8 + (d - 1)) as u8)
                } else {
                    ((i as i8 + d - 2) as u8, j)
                };
                Ok(Coord { i: ni, j: nj })
            }
        }?;
        {
            let index = res.into_index();
            self.maps[self.direction as usize]
                .get_mut(index)
                .ok_or(anyhow!("Out of bounds"))?
                .set(true);
        }
        if !with_food {
            for i in 0..4 {
                self.maps[i].set(self.tail.into_index(),false);
            }
        } 
        Ok(())
    }
}
