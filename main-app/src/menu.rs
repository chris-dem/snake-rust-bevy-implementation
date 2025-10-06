use bevy::prelude::*;

use crate::{
    AppState,
    common::{draw_button, label_bundle},
};

pub struct MenuPlugin;

const STATE: AppState = AppState::Menu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), draw_ui.run_if(in_state(STATE)))
            .add_systems(Update, draw_ui.run_if(in_state(STATE)));
    }
}

fn draw_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            DespawnOnExit(AppState::Menu),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder.spawn(label_bundle(
                "Snake AI".to_owned(),
                &asset_server,
                None,
                None,
            ));

            builder
                .spawn(draw_button("Start Game!".to_owned(), &asset_server))
                .observe(on_click);
        });
}

fn on_click(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Game);
}
