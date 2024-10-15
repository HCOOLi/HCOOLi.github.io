use bevy::prelude::*;

use crate::{Piece, PieceColor, Position};

#[derive(Resource)]
pub struct Board {
    board: [[Option<PieceColor>; 4]; 4],   // 棋盘
    current_player: PieceColor,            // 当前玩家
    pub selected_pieces: Option<Position>, // 被选中的棋子
    pub last_pieces: Option<Position>,     // 上一步
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [
                [Some(PieceColor::Black); 4],
                [None; 4],
                [None; 4],
                [Some(PieceColor::White); 4],
            ],
            current_player: PieceColor::Black,
            selected_pieces: None,
            last_pieces: None,
        }
    }
}

pub fn spawn_board(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let board_asset = asset_server.load("board.png");

    commands.spawn(SpriteBundle {
        texture: board_asset,
        transform: Transform::from_xyz(-20.0, 0.0, 0.0),
        ..default()
    });
    for x in 0..4 {
        let black_piece = asset_server.load("chess_black.png");
        let white_piece = asset_server.load("chess_white.png");
        commands
            .spawn(SpriteBundle {
                texture: black_piece,
                transform: Transform::from_xyz(x as f32 * 100.0 - 170.0, -150.0, 10.0),
                ..default()
            })
            .insert(Piece {
                color: PieceColor::Black,
                pos: Position::new(x, 0),
            });
        commands
            .spawn(SpriteBundle {
                texture: white_piece,
                transform: Transform::from_xyz(x as f32 * 100.0 - 170.0, 150.0, 10.0),
                ..default()
            })
            .insert(Piece {
                color: PieceColor::White,
                pos: Position::new(x, 3),
            });
    }
}
