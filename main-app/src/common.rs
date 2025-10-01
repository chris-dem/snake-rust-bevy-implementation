use bevy::prelude::*;
use snake_api_lib::common::Coord;

use crate::setup::WinDimension;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub(crate) struct Position(pub(crate) Coord);

pub(crate) fn pos_to_vec(
    coord: Coord,
    width: f32,
    height: f32,
    max_width: f32,
    max_height: f32,
) -> Vec2 {
    Vec2::new(
        coord.col as f32 * width - max_width / 2. + width / 2.,
        -(coord.row as f32 * height - max_height / 2.) + height / 2.,
    )
}

impl WinDimension {
    pub(crate) fn from_coord_to_pos(self, coord: Coord) -> Vec2 {
        let (cw, ch) = self.cell_dims();
        let (ww, wh) = self.window_dims();
        pos_to_vec(coord, cw, ch, ww, wh)
    }
}
