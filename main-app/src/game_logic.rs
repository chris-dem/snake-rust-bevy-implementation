use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_smud::prelude::*; // Wait will add randomness and show you one sec
use snake_api_lib::{
    GameAPI,
    common::{Coord, GRID_X, GRID_Y},
};

use crate::{common::Position, setup::WinDimension};

#[derive(Debug, Clone, Copy, Resource)]
pub(crate) struct GameState(pub(crate) GameAPI);

pub struct GamePlugin;

#[derive(Clone, PartialEq, Eq, Resource, Default)]
pub(crate) struct ShaderResourceSnake(pub(crate) Handle<Shader>);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_setup.after(crate::setup::setup));
    }
}

fn game_setup(
    mut commands: Commands,
    mut shaders: ResMut<Assets<Shader>>,
    win_dim: Res<WinDimension>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let game_api = GameAPI::new(Some(&mut rng));
    commands.insert_resource(GameState(game_api));
    // game_api.snake.h;

    let (w, h) = win_dim.cell_dims();
    let sdf = shaders.add_sdf_expr(win_dim.generate_sdf_string());
    let position = Coord {
        row: (GRID_X * 2/ 3 + 5) as u8,
        col: (GRID_Y / 2) as u8,
    };
    let trans = win_dim.from_coord_to_pos(position);
    commands.insert_resource(ShaderResourceSnake(sdf.clone()));
    commands.spawn((
        SmudShape {
            color: Color::WHITE,
            sdf: sdf,
            frame: Frame::Quad(w.max(h) * 1.2),
            ..default()
        },
        Position(position),
        Transform::from_xyz(trans.x, trans.y, 10.),
    ));
}
