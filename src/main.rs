mod board;
mod chess;
mod consts;
use bevy::{prelude::*, time::*, window::WindowResolution};
use board::*;
use chess::*;
use consts::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    });
}

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, (spawn_board, spawn_cursor))
        .add_systems(
            Update,
            (
                position_translation,
                cursor_movement,
                piece_movement,
                cursor_select,
            ),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                title: String::from("齐庄战棋"),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .run();
}
