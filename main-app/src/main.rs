use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::setup::CameraPlugin;

pub mod common;
pub mod setup;
pub mod snake;

#[derive(Debug, Clone, Copy, Component)]
pub struct SnakeHead;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_plugins(CameraPlugin)
        .run();
}
