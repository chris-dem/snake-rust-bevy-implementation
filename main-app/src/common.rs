use bevy::prelude::*;
use snake_api_lib::common::Coord;

use crate::{constants::TEXT_COLOR_TITLE, setup::WinDimension};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub(crate) struct Position(pub(crate) Coord);

pub(crate) fn pos_to_vec(
    coord: Coord,
    width: f32,
    height: f32,
    max_width: f32,
    max_height: f32,
) -> Vec2 {
    Vec2::new(
        coord.col as f32 * width - max_width / 2. + width / 2.,
        -(coord.row as f32) * height + max_height / 2. - height / 2.,
    )
}

impl WinDimension {
    pub(crate) fn from_coord_to_pos(self, coord: Coord) -> Vec2 {
        let (cw, ch) = self.cell_dims();
        let (ww, wh) = self.window_dims();
        pos_to_vec(coord, cw, ch, ww, wh)
    }
}

pub(crate) fn label_bundle(
    text: String,
    asset_server: &AssetServer,
    text_col: Option<Color>,
    font_size: Option<f32>,
) -> impl Bundle {
    let text_col = text_col.unwrap_or(TEXT_COLOR_TITLE.into());
    let font_size = font_size.unwrap_or(150.0);
    (
        Text::new(text),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size,
            ..default()
        },
        TextColor(text_col),
        TextShadow::default(),
    )
}

pub(crate) fn draw_button(text: String, asset_server: &AssetServer) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(250.),
            height: Val::Px(65.),
            border: UiRect::all(Val::Px(5.)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BorderRadius::MAX,
        BackgroundColor(Color::BLACK),
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}
