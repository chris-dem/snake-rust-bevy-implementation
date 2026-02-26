use std::{
    fmt::Display,
    ops::{Add, Sub},
};
use strum_macros::EnumIter;

pub const GRID_X: usize = 12;
pub const GRID_Y: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub row: i16,
    pub col: i16,
}

impl Default for Coord {
    fn default() -> Self {
        Self::middle()
    }
}

impl Coord {
    pub fn l1(self, other: Self) -> u16 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
    pub fn l0(self, other: Self) -> u16 {
        self.row
            .abs_diff(other.row)
            .max(self.col.abs_diff(other.col))
    }

    pub fn into_index(self) -> usize {
        (self.row as usize) * GRID_Y + self.col as usize
    }

    pub fn from_index(other: usize) -> Self {
        Self {
            row: (other % GRID_Y) as i16,
            col: (other / GRID_Y) as i16,
        }
    }

    pub fn add_dir(self, other: Direction) -> Self {
        let Coord { row, col } = self;
        let d = other as i16;
        if d % 2 == 0 {
            Coord {
                row,
                col: col + (d - 1),
            }
        } else {
            Coord {
                row: row + d - 2,
                col,
            }
        }
    }

    pub fn middle() -> Self {
        Self {
            row: GRID_Y as i16 / 2,
            col: GRID_X as i16 / 2,
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

impl Direction {
    pub fn inverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
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

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Left,
            1 => Self::Up,
            2 => Self::Right,
            3 => Self::Down,
            _ => unreachable!(),
        }
    }
}

pub enum Cell {
    Snake,
    Apple,
    Empty,
}
