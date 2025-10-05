use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_smud::prelude::*;

use crate::{
    endscreen::EndScreenPlugin,
    game_logic::GamePlugin,
    menu::MenuPlugin,
    setup::CameraPlugin,
};

pub(crate) mod common;
pub(crate) mod constants;
pub(crate) mod endscreen;
pub(crate) mod game_logic;
pub(crate) mod menu;
pub(crate) mod setup;
pub(crate) mod ui_handling;

#[derive(Debug, Clone, Copy, Default, States, PartialEq, Eq, Hash)]
pub(crate) enum AppState {
    #[default]
    Menu,
    Game,
    EndScreen,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SmudPlugin))
        .add_plugins(EntropyPlugin::<WyRand>::new())
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_plugins((CameraPlugin, MenuPlugin, EndScreenPlugin))
        .add_plugins(GamePlugin)
        .add_plugins(ui_handling::UiPlugin)
        .run();
}
