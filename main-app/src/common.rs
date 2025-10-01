use bevy::prelude::*;

use crate::setup::WinDimension;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Position {
    i: usize,
    j: usize,
}

impl Position {
    pub(crate) fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    pub(crate) fn from_win_dims_vec(self, win_dims: WinDimension) -> Vec2 {
        let (cw, ch) = win_dims.cell_dims();
        let (ww, wh) = win_dims.window_dims();
        self.pos_to_vec(cw, ch, ww, wh)
    }

    pub(crate) fn pos_to_vec(
        self,
        width: f32,
        height: f32,
        max_width: f32,
        max_height: f32,
    ) -> Vec2 {
        Vec2::new(
            self.j as f32 * width - max_width / 2. + width / 2.,
            self.i as f32 * height - max_height / 2. + height / 2.,
        )
    }
}
