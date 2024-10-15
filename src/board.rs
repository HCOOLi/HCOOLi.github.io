use bevy::{color::*, prelude::*, sprite::*};

use crate::{Piece, Position};

#[derive(Component)]
pub struct Board {
    board: [[u8; 4]; 4],
}

fn translate(v: [i32; 2]) -> Vec2 {
    Vec2::new(v[0] as f32 * 100.0 - 150.0, v[1] as f32 * 100.0 - 150.0)
}

pub fn spawn_board(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let board_asset = asset_server.load("board.png");

    commands.spawn(SpriteBundle {
        texture: board_asset,
        ..default()
    });
    for x in 0..4 {
        let black_piece = asset_server.load("chess_black.png");
        let white_piece = asset_server.load("chess_white.png");
        commands
            .spawn(SpriteBundle {
                texture: black_piece,
                transform: Transform::from_xyz(x as f32 * 100.0 - 150.0, -150.0, 10.0),
                ..default()
            })
            .insert(Position::new(x, 0))
            .insert(Piece);
        commands
            .spawn(SpriteBundle {
                texture: white_piece,
                transform: Transform::from_xyz(x as f32 * 100.0 - 150.0, 150.0, 10.0),
                ..default()
            })
            .insert(Position::new(x, 3))
            .insert(Piece);
    }
}
