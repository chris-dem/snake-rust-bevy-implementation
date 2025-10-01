use bevy::prelude::*;
use bevy_smud::prelude::*;

use crate::{game_logic::GamePlugin, setup::CameraPlugin};

pub mod common;
pub mod game_logic;
pub mod setup;
pub mod snake;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SmudPlugin))
        .add_plugins((CameraPlugin, GamePlugin))
        .run();
}
