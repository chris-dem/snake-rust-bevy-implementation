use bevy::color::{
    Srgba,
    palettes::tailwind::{PURPLE_400, TEAL_400, YELLOW_100},
};

pub(crate) const FRAME_MUL: f32 = 1.2;
pub(crate) const BLOCK_Z: f32 = 10.;
pub(crate) static SNAKE_COLOUR: Srgba = TEAL_400;
pub(crate) static APPLE_COLOUR: Srgba = PURPLE_400;
pub(crate) static TEXT_COLOR: Srgba = Srgba {
    alpha: 0.2,
    ..YELLOW_100
};
