use std::time::Duration;

use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_smud::prelude::*; // Wait will add randomness and show you one sec
use snake_api_lib::{
    GameAPI,
    common::{Coord, Direction},
    snake::SnakeTrait,
};

use crate::{
    common::Position,
    constants::{APPLE_COLOUR, BLOCK_Z, FRAME_MUL, SNAKE_COLOUR},
    setup::WinDimension,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub(crate) struct AppleComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub(crate) struct SnakeHeadComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub(crate) struct SnakeTailComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Default)]
pub(crate) struct NextEl(Option<Entity>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Resource, Default)]
pub(crate) struct FoodConsumed(pub(crate) bool);

#[derive(Debug, Clone, Copy, Resource)]
pub(crate) struct GameState(pub(crate) GameAPI);

pub struct GamePlugin;

#[derive(Clone, PartialEq, Eq, Resource, Default)]
pub(crate) struct ShaderResourceSnake(pub(crate) Handle<Shader>);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f64(0.5)))
            .init_resource::<FoodConsumed>()
            .add_systems(Startup, game_setup.after(crate::setup::setup))
            .add_systems(Update, set_keyboard_dir)
            .add_systems(FixedUpdate, step_snake)
            .add_systems(FixedUpdate, (ui_snake).after(step_snake));
    }
}

fn step_snake(mut snake_state: ResMut<GameState>, mut rng: GlobalEntropy<WyRand>) {
    let s = match snake_state.0.next(&mut rng) {
        Ok(state) => state,
        Err(e) => panic!("{}", e.to_string()),
    };
}

fn ui_snake(
    snake_state: Res<GameState>,
    win_dim: Res<WinDimension>,
    sdf_res: Res<ShaderResourceSnake>,
    mut query_head: Query<
        (&mut NextEl, Entity),
        (With<SnakeHeadComponent>, Without<SnakeTailComponent>),
    >,
    mut query_pos: Query<
        (&mut NextEl, Entity, &Position),
        Or<(
            (With<SnakeTailComponent>, Without<SnakeHeadComponent>),
            (With<SnakeTailComponent>, With<SnakeHeadComponent>),
        )>,
    >,
    mut commands: Commands,
) {
    {
        let mut qlens = query_pos
                .transmute_lens_filtered::<(&mut NextEl, Entity),
                    (With<SnakeTailComponent>, With<SnakeHeadComponent>)>();
        let mut query = qlens.query();
        let x = query.single_mut().ok();
        let Some(mut head) = query_head.single_mut().ok().or(x) else {
            panic!("Head mising");
        };

        commands.entity(head.1).remove::<SnakeHeadComponent>();
        let ent = commands
            .spawn(draw_cell(
                snake_state.0.snake.head,
                *win_dim,
                sdf_res.0.clone(),
                SNAKE_COLOUR.into(),
            ))
            .insert((NextEl(None), SnakeHeadComponent))
            .id();
        head.0.0 = Some(ent);
    }

    let Ok((tail_next, tail_ent, tail_pos)) = query_pos.single() else {
        panic!("Tail missing");
    };
    if tail_pos.0 != snake_state.0.snake.tail {
        // If food not eaten
        if let Some(next_ent) = tail_next.0 {
            commands.entity(next_ent).insert(SnakeTailComponent);
            commands.entity(tail_ent).despawn();
        } else {
            error!("Tail next not assigned")
        }
    }
}

fn ui_apple(
    snake_state: Res<GameState>,
    win_dim: Res<WinDimension>,
    sdf_res: Res<ShaderResourceSnake>,
    mut apple_flag: ResMut<FoodConsumed>,
    query_pos: Single<(&Position, Entity), With<AppleComponent>>,
    mut commands: Commands,
) {
    let (apple_pos, apple_ent) = *query_pos;
    if apple_pos.0 != snake_state.0.apples {
        // Redrarw
        commands.entity(apple_ent).despawn();
        commands
            .spawn(draw_cell(
                snake_state.0.apples,
                *win_dim,
                sdf_res.0.clone(),
                APPLE_COLOUR.into(),
            ))
            .insert(AppleComponent);
        apple_flag.0 = true;
    }
}

fn set_keyboard_dir(mut res: ResMut<GameState>, key: Res<ButtonInput<KeyCode>>) {
    let dir = if key.any_just_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        Direction::Left
    } else if key.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        Direction::Up
    } else if key.any_just_pressed([KeyCode::KeyD, KeyCode::ArrowDown]) {
        Direction::Down
    } else if key.any_just_pressed([KeyCode::ArrowRight, KeyCode::KeyS]) {
        Direction::Right
    } else {
        return;
    };
    res.0.snake.direction(dir);
}

fn draw_cell(
    coord: Coord,
    win_dims: WinDimension,
    sdf_handle: Handle<Shader>,
    color: Color,
) -> impl Bundle {
    let trans = win_dims.from_coord_to_pos(coord);
    let frame_dim = {
        let (w, h) = win_dims.cell_dims();
        w.max(h) * FRAME_MUL
    };

    (
        SmudShape {
            color,
            sdf: sdf_handle.clone(),
            frame: Frame::Quad(frame_dim),
            ..default()
        },
        Position(coord),
        Transform::from_xyz(trans.x, trans.y, BLOCK_Z),
    )
}

fn game_setup(
    mut commands: Commands,
    mut shaders: ResMut<Assets<Shader>>,
    win_dim: Res<WinDimension>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let game_api = GameAPI::new(Some(&mut rng));
    commands.insert_resource(GameState(game_api));

    let sdf = shaders.add_sdf_expr(win_dim.generate_sdf_string());
    let position_head = game_api.snake.head;
    let position_apple = game_api.apples;
    commands.insert_resource(ShaderResourceSnake(sdf.clone()));
    commands
        .spawn(draw_cell(
            position_head,
            *win_dim,
            sdf.clone(),
            SNAKE_COLOUR.into(),
        ))
        .insert((SnakeHeadComponent, SnakeTailComponent, NextEl(None)));
    commands
        .spawn(draw_cell(
            position_apple,
            *win_dim,
            sdf.clone(),
            APPLE_COLOUR.into(),
        ))
        .insert(AppleComponent);
}
