use bevy::{color::palettes::css::RED, prelude::*};
use bevy_smud::prelude::*;
use snake_api_lib::{
    GameAPI,
    common::{GRID_X, GRID_Y},
};

use crate::{common::Position, setup::WinDimension};

#[derive(Debug, Clone, Copy, Component)]
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
) {
    let (w, h) = win_dim.cell_dims();

    let sdf = shaders.add_sdf_expr(win_dim.generate_sdf_string());
    let position = Position::new(GRID_Y, GRID_X/2);
    let trans = dbg!(position.from_win_dims_vec(*win_dim));
    commands.insert_resource(ShaderResourceSnake(sdf.clone()));
    commands.spawn((
        SmudShape {
            color: Color::WHITE,
            sdf: sdf,
            frame: Frame::Quad(w.max(h) * 1.2),
            ..default()
        },
        position,
        Transform::from_xyz(trans.x, trans.y, 10.),
    ));
}
