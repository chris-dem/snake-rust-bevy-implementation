use bevy::color::{
    palettes::tailwind::{PURPLE_400, TEAL_400, YELLOW_100, YELLOW_500}, Srgba
};

pub(crate) const FRAME_MUL: f32 = 1.2;
pub(crate) const BLOCK_Z: f32 = 10.;
pub(crate) static SNAKE_COLOUR: Srgba = TEAL_400;
pub(crate) static APPLE_COLOUR: Srgba = PURPLE_400;
pub(crate) const TEXT_COLOR: Srgba = Srgba {
    alpha: 0.2,
    ..YELLOW_100
};
pub(crate) const TEXT_COLOR_TITLE: Srgba = YELLOW_500;