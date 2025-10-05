use bevy::prelude::*;

use crate::{
    AppState,
    common::{draw_button, label_bundle},
    constants::TEXT_COLOR_TITLE,
};

pub(crate) struct EndScreenPlugin;

impl Plugin for EndScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<EndScreenState>()
            .add_systems(OnEnter(AppState::EndScreen), draw_ui);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SubStates, Default)]
#[source(AppState = AppState::EndScreen)]
pub(crate) enum EndScreenState {
    Win,
    #[default]
    Lose,
}

fn draw_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            StateScoped(AppState::EndScreen),
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
                Some(TEXT_COLOR_TITLE.into()),
                Some(150.),
            ));

            builder
                .spawn(draw_button("Start Game!".to_owned(), &asset_server))
                .observe(on_click);
        });
}

fn on_click(_: Trigger<Pointer<Click>>) {}
