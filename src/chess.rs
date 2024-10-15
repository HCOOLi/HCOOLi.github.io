use crate::consts::*;
use bevy::{color::*, prelude::*, sprite::*};
// 键盘移动的光标
#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct Piece;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        if x < 0 || x > 3 || y < 0 || y > 3 {
            panic!("out of range")
        }
        Position { x, y }
    }

    pub fn move_up(&mut self) {
        if self.y < 3 {
            self.y += 1;
        }
    }
    pub fn move_down(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < 3 {
            self.x += 1;
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
}

pub fn spawn_cursor(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let cursor_image = asset_server.load("cursor.png");
    commands
        .spawn(SpriteBundle {
            texture: cursor_image,
            transform: Transform {
                scale: Vec3::new(0.5, 0.5, 0.0),
                translation: Vec3::new(0.0, 0.0, 20.0),
                ..default()
            },
            ..default()
        })
        .insert(Cursor)
        .insert(Position { x: 0, y: 0 });
}

pub enum PlayerColor {
    Black,
    White,
}

pub fn position_translation(
    windows: Query<&mut Window>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32) -> f32 {
        let tile_size = bound_window / 5.0;
        pos / 5.0 * bound_window - (bound_window / 2.) + tile_size
    }
    let window = windows.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32),
            convert(pos.y as f32, window.height() as f32),
            5.0,
        );
    }
}

pub fn cursor_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cursor_positions: Query<&mut Position, With<Cursor>>,
) {
    for mut pos in cursor_positions.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::ArrowLeft)
            || keyboard_input.just_pressed(KeyCode::KeyA)
        {
            pos.move_left();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight)
            || keyboard_input.just_pressed(KeyCode::KeyD)
        {
            pos.move_right();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown)
            || keyboard_input.just_pressed(KeyCode::KeyS)
        {
            pos.move_down();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp)
            || keyboard_input.just_pressed(KeyCode::KeyW)
        {
            pos.move_up();
        }
    }
}

pub fn cursor_select(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cursor_positions: Query<&mut Position, With<Cursor>>,
    mut piece_positions: Query<&mut Position, With<Piece>>,
) {
    for mut pos in cursor_positions.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Enter) {
            let pos_1 = pos.clone();
            for mut pie in piece_positions.iter_mut() {
                let pie_1 = pie.clone();
                if pos_1 == pie_1 {
                    pos.move_up();
                    pie.move_up();
                }
            }
        }
    }
}

pub fn piece_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut piece_positions: Query<&mut Position, With<Piece>>,
) {
    for mut pos in piece_positions.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::ArrowLeft)
            || keyboard_input.just_pressed(KeyCode::KeyA)
        {
            pos.move_left();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight)
            || keyboard_input.just_pressed(KeyCode::KeyD)
        {
            pos.move_right();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown)
            || keyboard_input.just_pressed(KeyCode::KeyS)
        {
            pos.move_down();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp)
            || keyboard_input.just_pressed(KeyCode::KeyW)
        {
            pos.move_up();
        }
    }
}

pub fn handle_input(mouse: Res<ButtonInput<MouseButton>>) {}
