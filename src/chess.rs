use crate::Board;
use bevy::{color::*, prelude::*};

// 键盘移动的光标
#[derive(Component)]
pub struct Cursor;

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        }
    }
}

#[derive(Component)]
pub struct CurrentPlayer;

#[derive(Bundle)]
pub struct Piece {
    pub color: PieceColor,
    pub pos: Position,
    // pub selected: bool,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
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
    // let black_img = asset_server.load("chess_black.png");
    // let white_img = asset_server.load("chess_white.png");
    commands
        .spawn(SpriteBundle {
            texture: cursor_image,
            transform: Transform {
                scale: Vec3::new(0.6, 0.6, 0.0),
                translation: Vec3::new(0.0, 0.0, 40.0),
                ..default()
            },
            ..default()
        })
        .insert(Cursor)
        .insert(Position { x: 0, y: 0 });
}

pub fn spawn_current_player(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "current\nplayer",
            TextStyle {
                font_size: 20.0,
                color: Srgba::WHITE.into(),
                ..default()
            },
        )
        .with_justify(JustifyText::Left),
        transform: Transform::from_xyz(200.0, 200.0, 20.0),
        ..default()
    });
    let black_img = asset_server.load("chess_black.png");
    let white_img = asset_server.load("chess_white.png");
    commands
        .spawn(SpriteBundle {
            texture: black_img,
            transform: Transform {
                translation: Vec3::new(200.0, 150.0, 20.0),
                ..default()
            },
            ..default()
        })
        .insert(CurrentPlayer);

    commands
        .spawn(SpriteBundle {
            texture: white_img,
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(200.0, 150.0, 20.0),
                ..default()
            },
            ..default()
        })
        .insert(CurrentPlayer);
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
            convert(pos.x as f32, window.width() as f32) - 20.0,
            convert(pos.y as f32, window.height() as f32),
            transform.translation.z,
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
    cursor_positions: Query<&Position, With<Cursor>>,
    mut piece_positions: Query<
        (&mut Position, &mut Transform),
        (With<PieceColor>, Without<Cursor>),
    >,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if let Some(selected) = board.selected_pieces {
            println!("selected pos :{:?}", selected);
            for (mut pie, mut trans) in piece_positions.iter_mut() {
                if selected == *pie {
                    if let Ok(cur) = cursor_positions.get_single() {
                        println!("cursor pos :{:?}", cur);
                        *pie = *cur;
                        board.selected_pieces = None;
                        trans.scale = Vec3::new(1.0, 1.0, 0.0);
                    }
                    println!("moved");
                    return;
                }
            }
        } else {
            if let Ok(cur) = cursor_positions.get_single() {
                println!("cursor pos :{:?}", cur);
                for (pie, mut trans) in piece_positions.iter_mut() {
                    println!("piece pos :{:?}", pie);
                    let pie_1 = pie;
                    if *cur == *pie_1 {
                        println!("selected");
                        trans.scale = Vec3::new(1.2, 1.2, 0.0);
                        board.selected_pieces = Some(cur.clone());
                        return;
                    }
                }
            }
        }
    }
}

// pub fn piece_movement(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut piece_positions: Query<&mut Position, With<PieceColor>>,
//     mut board: ResMut<Board>,
// ) {
//     if let Some(pos) = board.selected_pieces {
//         for mut pie_pos in piece_positions.iter_mut() {
//             if *pie_pos == pos {
//                 if keyboard_input.just_pressed(KeyCode::ArrowLeft)
//                     || keyboard_input.just_pressed(KeyCode::KeyA)
//                 {
//                     pie_pos.move_left();
//                     board.selected_pieces = Some(pie_pos.clone());
//                 }
//                 if keyboard_input.just_pressed(KeyCode::ArrowRight)
//                     || keyboard_input.just_pressed(KeyCode::KeyD)
//                 {
//                     pie_pos.move_right();
//                     board.selected_pieces = Some(pie_pos.clone());
//                 }
//                 if keyboard_input.just_pressed(KeyCode::ArrowDown)
//                     || keyboard_input.just_pressed(KeyCode::KeyS)
//                 {
//                     pie_pos.move_down();
//                     board.selected_pieces = Some(pie_pos.clone());
//                 }
//                 if keyboard_input.just_pressed(KeyCode::ArrowUp)
//                     || keyboard_input.just_pressed(KeyCode::KeyW)
//                 {
//                     pie_pos.move_up();
//                     board.selected_pieces = Some(pie_pos.clone());
//                 }
//             }
//         }
//     }
// }

pub fn handle_input(mouse: Res<ButtonInput<MouseButton>>) {
    todo!("实现鼠标逻辑")
}
