use bevy::{prelude::*, window::WindowResized};
use bevy_smud::prelude::*;
use snake_api_lib::common::{GRID_X, GRID_Y};

use crate::{common::*, game_logic::ShaderResourceSnake};

#[derive(Debug, Clone, Copy, Resource, PartialEq)]
pub struct WinDimension(f32, f32);

impl WinDimension {
    pub fn window_dims(self) -> (f32, f32) {
        (self.0, self.1)
    }

    pub fn cell_dims(self) -> (f32, f32) {
        (self.0 / GRID_Y as f32, self.1 / GRID_X as f32)
    }

    pub fn generate_sdf_string(self) -> String {
        let (w, h) = self.cell_dims();
        format!("smud::sd_box(p, vec2<f32>({},{}))", w * 0.5, h * 0.5).to_owned()
    }
}

pub(crate) fn setup(mut commands: Commands, win: Single<&Window>) {
    commands.spawn(Camera2d);
    commands.insert_resource(WinDimension(win.width(), win.height()));
}

fn update_win(
    mut event_reader: EventReader<WindowResized>,
    mut win_dims: ResMut<WinDimension>,
    mut snake_shader: ResMut<ShaderResourceSnake>,
    mut shaders: ResMut<Assets<Shader>>,
    mut query_shapes: Query<(&mut SmudShape, &mut Transform, &Position)>,
) {
    for WindowResized { width, height, .. } in event_reader.read() {
        win_dims.0 = *width;
        win_dims.1 = *height;
        let new_handle = shaders.add_sdf_expr(win_dims.generate_sdf_string());
        snake_shader.0 = new_handle.clone();
        for (mut shape, mut trans, pos) in query_shapes.iter_mut() {
            shape.sdf = new_handle.clone();
            let (w, h) = win_dims.cell_dims();
            shape.frame = Frame::Quad(w.max(h) * 1.2);
            let new_pos = win_dims.from_coord_to_pos(pos.0);
            trans.translation.x = new_pos.x;
            trans.translation.y = new_pos.y;
        }
    }
}

fn debug_grid(mut gizmo: Gizmos, win_dims: Res<WinDimension>) {
    let (cell_w, cell_h) = win_dims.cell_dims();
    gizmo
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(GRID_Y as u32, GRID_X as u32),
            Vec2::new(cell_w, cell_h),
            // Dark gray
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, debug_grid)
            .add_systems(Update, update_win);
    }
}
