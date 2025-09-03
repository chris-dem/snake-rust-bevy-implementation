use bevy::prelude::*;
pub const GRID_WIDTH: usize = 40;
pub const GRID_HEIGHT: usize = 30;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Position {
    i: usize,
    j: usize,
}

impl Position {
    pub(crate) fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    pub(crate) fn pos_to_vec(
        self,
        width: f32,
        height: f32,
        max_width: f32,
        max_height: f32,
    ) -> Vec2 {
        Vec2::new(
            self.i as f32 * width - max_width / 2. + width / 2.,
            self.j as f32 * height - max_height / 2. + height / 2.,
        )
    }
}
