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
    fn move_up(&mut self) {
        if self.y < 3 {
            self.y += 1;
        }
    }
    fn move_down(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
    fn move_right(&mut self) {
        if self.x < 3 {
            self.x += 1;
        }
    }
    fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
}

pub fn spawn_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle_mesh = Mesh2dHandle(meshes.add(Circle { radius: 30.0 }));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: circle_mesh,
            material: materials.add(ColorMaterial::from_color(palettes::css::OLIVE)),
            transform: Transform {
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

// fn spawn_pieces(mut commands: Commands) {
//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: SNAKE_HEAD_COLOR,
//                 ..default()
//             },
//             transform: Transform {
//                 scale: Vec3::new(10.0, 10.0, 10.0),
//                 ..default()
//             },
//             ..default()
//         });
// }

pub fn cursor_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Position, With<Cursor>>,
) {
    for mut pos in head_positions.iter_mut() {
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
