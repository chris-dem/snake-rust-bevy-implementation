use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_smud::prelude::*;

use crate::{game_logic::GamePlugin, setup::CameraPlugin};

pub(crate) mod common;
pub(crate) mod constants;
pub(crate) mod game_logic;
pub(crate) mod setup;
pub(crate) mod ui_handling;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SmudPlugin))
        .add_plugins(EntropyPlugin::<WyRand>::new())
        .add_plugins((CameraPlugin, GamePlugin, ui_handling::UiPlugin))
        .run();
}
