use bevy::prelude::*;

use crate::{AppState, constants::TEXT_COLOR, game_logic::GameState};

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), ui_setup)
            .add_systems(
                Update,
                (draw_direction, draw_score)
                    .run_if(resource_exists_and_changed::<GameState>)
                    .run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct DirUi;

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct ScoreUi;

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut c: Color = TEXT_COLOR.into();
    c.set_alpha(0.1);

    commands.spawn((
        Text::new("->"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        },
        TextColor(c),
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        },
        DirUi,
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        },
        TextColor(c),
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(5.),
            ..default()
        },
        ScoreUi,
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        Text::new("Difficulty: Easy"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        },
        TextColor(c),
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        },
        ScoreUi,
        StateScoped(AppState::Game),
    ));
}

fn draw_direction(game: Res<GameState>, mut query_text: Single<&mut Text, With<DirUi>>) {
    query_text.0 = game.0.snake.direction.to_string();
}

fn draw_score(game: Res<GameState>, mut query_text: Single<&mut Text, With<ScoreUi>>) {
    query_text.0 = format!("Score: {}", game.0.score);
}

fn draw_difficulty(game: Res<GameState>, mut query_text: Single<&mut Text, With<ScoreUi>>) {
    query_text.0 = format!("Difficulty: {}", game.0.mode);
}
