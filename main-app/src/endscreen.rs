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

fn draw_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    end_state: Res<State<EndScreenState>>,
) {
    commands
        .spawn((
            DespawnOnExit(AppState::EndScreen),
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
                "Game Over".to_owned(),
                &asset_server,
                Some(TEXT_COLOR_TITLE.into()),
                Some(150.),
            ));

            let label = match end_state.get() {
                EndScreenState::Win => "You Won",
                EndScreenState::Lose => "You Lost",
            };

            builder.spawn(label_bundle(
                label.to_owned(),
                &asset_server,
                Some(TEXT_COLOR_TITLE.into()),
                Some(100.),
            ));

            builder
                .spawn(draw_button("Back to menu".to_owned(), &asset_server))
                .observe(on_click);
        });
}

fn on_click(_: On<Pointer<Click>>, mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Menu);
}
