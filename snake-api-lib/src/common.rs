use std::{
    fmt::Display,
    ops::{Add, Sub},
};
use strum_macros::EnumIter;

// pub const GRID_X: usize = 10;
pub const GRID_X: usize = 40;
pub const GRID_Y: usize = 32;
// pub const GRID_Y: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub row: u8,
    pub col: u8,
}

impl Default for Coord {
    fn default() -> Self {
        Self::middle()
    }
}

impl Coord {
    pub fn l1(self, other: Self) -> u8 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
    pub fn l0(self, other: Self) -> u8 {
        self.row
            .abs_diff(other.row)
            .max(self.col.abs_diff(other.col))
    }

    pub fn into_index(self) -> usize {
        (self.row as usize) * GRID_Y + self.col as usize
    }

    pub fn middle() -> Self {
        Self {
            row: GRID_Y as u8 / 2,
            col: GRID_X as u8 / 2,
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row.saturating_sub(rhs.row),
            col: self.col.saturating_sub(rhs.col),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Default)]
pub enum Direction {
    #[default]
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Up => '↑',
            Self::Right => '→',
            Self::Down => '↓',
            Self::Left => '←',
        };
        write!(f, "{c}")
    }
}

pub enum Cell {
    Snake,
    Apple,
    Empty,
}
