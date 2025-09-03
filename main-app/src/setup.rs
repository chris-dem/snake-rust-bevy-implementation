use bevy::{prelude::*, window::WindowResized};

use crate::common::*;

#[derive(Debug, Clone, Copy, Resource, PartialEq)]
pub struct WinDimension(f32, f32);

impl WinDimension {
    fn window_dims(self) -> (f32, f32) {
        (self.0, self.1)
    }

    fn cell_dims(self) -> (f32, f32) {
        (self.0 / GRID_WIDTH as f32, self.1 / GRID_HEIGHT as f32)
    }
}

fn setup(mut commands: Commands, win: Single<&Window>) {
    commands.spawn(Camera2d);
    commands.insert_resource(WinDimension(win.width(), win.height()));
}

fn update_win(mut event_reader: EventReader<WindowResized>, mut win_dims: ResMut<WinDimension>) {
    for WindowResized { width, height, .. } in event_reader.read() {
        win_dims.0 = *width;
        win_dims.1 = *height;
    }
}

fn debug_grid(mut gizmo: Gizmos, win_dims: Res<WinDimension>) {
    let (cell_w, cell_h) = win_dims.cell_dims();
    gizmo
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
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
