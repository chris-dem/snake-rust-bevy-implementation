use std::fmt::{Debug, Display};

use crate::common::{Coord, Direction, GRID_X, GRID_Y};
use anyhow::{Result as AResult, anyhow};
use bitvec::prelude::*;
use itertools::Itertools;
use strum::IntoEnumIterator;

// Fixed-size bit array
type GridBits = BitArr!(for GRID_X * GRID_Y, in u64); // 768 bits using u64 storage

pub trait SnakeTrait: Debug + Sized {
    fn check_cell(&self, coords: Coord) -> Option<bool>;
    fn direction(&mut self, dir: Direction);
    fn step(&mut self, with_food: bool) -> AResult<()>;
    fn is_next_valid(&self) -> bool;
    fn get_elements(&self) -> Vec<bool>;
}

#[derive(Debug, Clone, Copy)]
pub struct ArrSnake {
    maps: [GridBits; 4],
    direction: Direction,
    head: Coord,
    tail: Coord,
}

impl Default for ArrSnake {
    fn default() -> Self {
        let mut maps: [GridBits; 4] = Default::default();
        let middle = Coord::middle();
        let def_dir: Direction = Default::default();
        maps[def_dir as usize].set(middle.into_index(), true);
        Self {
            maps,
            direction: def_dir,
            head: middle,
            tail: middle,
        }
    }
}

impl Display for ArrSnake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n Head {:?} Tail {:?}", self.head, self.tail)?;
        write!(f, " \\|")?;
        for i in 0..GRID_Y {
            write!(f, "{:^3}|", i + 1)?;
        }
        writeln!(f)?;
        for i in 0..GRID_X {
            let i_print = i + 1;
            write!(f, "{i_print:^2}|")?;
            for j in 0..GRID_Y {
                let indx = Coord {
                    i: i as u8,
                    j: j as u8,
                }
                .into_index();
                let direction_vecs = [
                    self.maps[Direction::Left as usize][indx],
                    self.maps[Direction::Right as usize][indx],
                    self.maps[Direction::Up as usize][indx],
                    self.maps[Direction::Down as usize][indx],
                ];
                let c = match direction_vecs {
                    [true, false, false, false] => 'L',
                    [false, true, false, false] => 'R',
                    [false, false, true, false] => 'U',
                    [false, false, false, true] => 'D',
                    [false, false, false, false] => ' ',
                    _ => 'X',
                };
                write!(f, "{c:^3}|")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl ArrSnake {
    fn next_step(&self, coord: Coord, direction: Direction) -> AResult<Coord> {
        let ind = coord.into_index();
        if ind > GRID_X * GRID_Y {
            return Err(anyhow!("Out of bounds"));
        }

        match (coord, direction) {
            (Coord { j: 0, .. }, Direction::Left) | (Coord { i: 0, .. }, Direction::Up) => Err(
                anyhow!("Invalid coordinate {:?}, {:?}", coord, self.direction),
            ),
            (Coord { i, .. }, Direction::Down) if i == GRID_X as u8 => Err(anyhow!(
                "Invalid coordinate {:?}, {:?}",
                coord,
                self.direction
            )),
            (Coord { j, .. }, Direction::Right) if j == GRID_Y as u8 => Err(anyhow!(
                "Invalid coordinate {:?}, {:?}",
                coord,
                self.direction
            )),
            (Coord { i, j }, d) => {
                let d = d as i8;
                let (ni, nj) = if d % 2 == 0 {
                    (i, (j as i8 + (d - 1)) as u8)
                } else {
                    ((i as i8 + d - 2) as u8, j)
                };
                Ok(Coord { i: ni, j: nj })
            }
        }
    }
}

impl SnakeTrait for ArrSnake {
    fn is_next_valid(&self) -> bool {
        self.next_step(self.head, self.direction)
            .ok()
            .and_then(|e| self.check_cell(e))
            .is_some_and(|x| !x)
    }

    fn check_cell(&self, coords: Coord) -> Option<bool> {
        let indx = coords.into_index();
        if indx > GRID_X * GRID_Y {
            return None;
        }
        Some(self.maps.iter().all(|arr| arr[indx]))
    }

    fn direction(&mut self, dir: Direction) {
        self.direction = dir;
        for map in self.maps.iter_mut() {
            map.set(self.head.into_index(), false);
        }
        self.maps[dir as usize].set(self.head.into_index(), true);
    }

    fn step(&mut self, with_food: bool) -> AResult<()> {
        let res = self.next_step(self.head, self.direction)?;
        let index = res.into_index();
        {
            let mut ind = self.maps[self.direction as usize]
                .get_mut(index)
                .ok_or(anyhow!("Out of bounds"))?;
            *ind = true;
        }
        self.head = res;
        if !with_food {
            let tail_index = self.tail.into_index();
            for dir in Direction::iter() {
                if self.maps[dir as usize][tail_index] {
                    self.tail = self.next_step(self.tail, dir)?;
                }
                self.maps[dir as usize].set(tail_index, false);
            }
        }
        Ok(())
    }

    fn get_elements(&self) -> Vec<bool> {
        (self.maps[0] | self.maps[1] | self.maps[2] | self.maps[3])
            .into_iter()
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_contain_in_middle_start() {
        let snake = ArrSnake::default();
        let els = snake.get_elements();
        assert_eq!(els.len(), GRID_X * GRID_Y);
        let middle = Coord::middle();
        let rest = els
            .iter()
            .enumerate()
            .filter(|(ind, _)| *ind != middle.into_index())
            .all(|(_, b)| *b == false);
        assert!(rest);
        assert_eq!(snake.direction, Direction::Left);
        assert!(els[middle.into_index()], "Middle should be true on init");
    }

    #[test]
    fn step_one_left_from_beginning_no_food() {
        let mut snake = ArrSnake::default();
        snake.step(false).expect("Should step normally");
        let middle = Coord::middle() - Coord { i: 0, j: 1 };
        let els = snake.get_elements();
        assert_eq!(els.len(), GRID_X * GRID_Y);
        let rest = els
            .iter()
            .enumerate()
            .filter(|(ind, _)| *ind != middle.into_index())
            .all(|(_, b)| *b == false);
        assert_eq!(snake.head, middle);
        assert_eq!(snake.tail, middle);
        assert!(rest);
        assert_eq!(snake.direction, Direction::Left);
        assert!(els[middle.into_index()], "Middle should be true on init");
    }

    #[test]
    fn step_one_left_from_beginning_food() {
        let mut snake = ArrSnake::default();
        snake.step(true).expect("Should step normally");
        let middle = Coord::middle() - Coord { i: 0, j: 1 };
        let els = snake.get_elements();
        assert_eq!(els.len(), GRID_X * GRID_Y);
        let rest = els
            .iter()
            .enumerate()
            .filter(|(ind, _)| ![Coord::middle().into_index(), middle.into_index()].contains(ind))
            .all(|(_, b)| !*b);
        assert_eq!(snake.head, middle);
        assert_eq!(snake.tail, Coord::middle());
        assert!(rest);
        assert_eq!(snake.direction, Direction::Left);
        assert!(els[middle.into_index()], "Middle should be true on init");
        assert!(
            els[Coord::middle().into_index()],
            "Middle should be true on init"
        );
        println!("{}", snake);
    }

    #[test]
    fn step_one_left_from_beginning_food_and_up() {
        let mut snake = ArrSnake::default();
        snake.step(true).expect("Should step normally");
        snake.direction(Direction::Up);
        snake.step(false).expect("Should step normally");
        let left = Coord::middle() - Coord { i: 0, j: 1 };
        let up = left - Coord { i: 1, j: 0 };
        let els = snake.get_elements();
        assert_eq!(els.len(), GRID_X * GRID_Y);
        let rest = els
            .iter()
            .enumerate()
            .filter(|(ind, _)| ![left.into_index(), up.into_index()].contains(ind))
            .all(|(_, b)| !*b);
        assert_eq!(snake.head, up);
        assert_eq!(snake.tail, left);
        assert!(rest);
        assert_eq!(snake.direction, Direction::Up);
        assert!(els[up.into_index()], "Middle should be true on init");
        assert!(els[left.into_index()], "Middle should be true on init");
        println!("{}", snake);
    }

    #[test]
    fn step_zig_zag() {
        let mut snake = ArrSnake::default();
        snake.step(true).expect("Should step normally");
        snake.direction(Direction::Up);
        snake.step(true).expect("Should step normally");
        snake.direction(Direction::Right);
        snake.step(false).expect("Should step normally");
        println!("{}", snake);
    }
}
