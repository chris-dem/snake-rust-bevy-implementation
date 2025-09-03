pub const GRID_X: usize = 32;
pub const GRID_Y: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub i: u8,
    pub j: u8,
}

impl Default for Coord {
    fn default() -> Self {
        Self::middle()
    }
}

impl Coord {
    pub fn l1(self, other: Self) -> u8 {
        self.i.abs_diff(other.i) + self.j.abs_diff(other.j)
    }
    pub fn l0(self, other: Self) -> u8 {
        self.i.abs_diff(other.i).max(self.j.abs_diff(other.j))
    }

    pub fn into_index(self) -> usize {
        (self.i as usize) * GRID_Y + self.j as usize
    }

    pub fn middle() -> Self {
        Self {
            i: GRID_X as u8 / 2,
            j: GRID_Y as u8 / 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Direction {
    #[default]
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

pub enum Cell {
    Snake,
    Apple,
    Empty,
}
