use bevy::prelude::*;

use crate::{Piece, PieceColor, Position};

#[derive(Resource)]
pub struct Board {
    board: [[Option<PieceColor>; 4]; 4], // 棋盘
    current_player: PieceColor,          // 当前玩家
    pub selected_pieces: Option<u32>,    // 被选中的棋子的EntityID
    pub last_pieces: Option<Position>,   // 上一步
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[Some(PieceColor::Black), None, None, Some(PieceColor::White)]; 4],
            current_player: PieceColor::Black,
            selected_pieces: None,
            last_pieces: None,
        }
    }
    fn index(&self, (x, y): (i32, i32)) -> Result<Option<PieceColor>, String> {
        let err1 = Err(String::from("out of range"));
        if x < 0 || x >= 4 || y < 0 || y >= 4 {
            return err1;
        }
        let (x, y) = (x as usize, y as usize);
        return Ok(self.board[x][y]);
    }

    fn set(&mut self, (x, y): (i32, i32), c: Option<PieceColor>) -> Result<(), String> {
        let err1 = Err(String::from("out of range"));
        if x < 0 || x >= 4 || y < 0 || y >= 4 {
            return err1;
        }
        let (x, y) = (x as usize, y as usize);
        self.board[x][y] = c;
        return Ok(());
    }

    pub fn print(&self) {
        for x in self.board {
            for y in x {
                print!("{:?} ", y.unwrap())
            }
            println!()
        }
    }

    pub fn move_chess(&mut self, from: (i32, i32), to: (i32, i32)) -> Result<(), String> {
        let err1 = Err(String::from("too far"));
        let err2 = Err(String::from("already has one"));
        let err3 = Err(String::from("empty"));
        let abs = |x: i32| -> i32 {
            if x < 0 {
                -x
            } else {
                x
            }
        };
        if abs(to.0 - from.0) + abs(to.1 - from.1) > 1 {
            return err1;
        }

        let current = self.index(from)?;
        if current == None {
            return err3;
        }
        let next = self.index(to)?;
        if next != None {
            return err2;
        }
        self.set(from, None)?;
        self.set(to, current)?;
        return Ok(());
    }
    pub fn eat_piece(&mut self, p: (i32, i32)) {
        self.set(p, None);
    }

    fn can_eat(
        &self,
        x1: Option<PieceColor>,
        x2: Option<PieceColor>,
        x3: Option<PieceColor>,
        x4: Option<PieceColor>,
    ) -> Option<i32> {
        match (x1, x2, x3, x4) {
            (
                Some(PieceColor::Black),
                Some(PieceColor::White),
                Some(PieceColor::White),
                None | Some(PieceColor::White),
            ) => Some(0),
            (
                Some(PieceColor::White),
                Some(PieceColor::Black),
                Some(PieceColor::Black),
                None | Some(PieceColor::Black),
            ) => Some(0),
            (
                None | Some(PieceColor::White),
                Some(PieceColor::Black),
                Some(PieceColor::White),
                Some(PieceColor::White),
            ) => Some(1),
            (
                None | Some(PieceColor::Black),
                Some(PieceColor::White),
                Some(PieceColor::Black),
                Some(PieceColor::Black),
            ) => Some(1),
            (
                Some(PieceColor::Black),
                Some(PieceColor::Black),
                Some(PieceColor::White),
                None | Some(PieceColor::Black),
            ) => Some(2),
            (
                Some(PieceColor::White),
                Some(PieceColor::White),
                Some(PieceColor::Black),
                None | Some(PieceColor::White),
            ) => Some(2),
            (
                None | Some(PieceColor::White),
                Some(PieceColor::White),
                Some(PieceColor::White),
                Some(PieceColor::Black),
            ) => Some(3),
            (
                None | Some(PieceColor::Black),
                Some(PieceColor::Black),
                Some(PieceColor::Black),
                Some(PieceColor::White),
            ) => Some(3),
            _ => None,
        }
    }
    pub fn can_eat_chess(&mut self, (x, y): (i32, i32)) -> Vec<(i32, i32)> {
        let mut res = Vec::new();
        let (p1, p2, p3, p4) = (
            self.index((x, 0)).unwrap(),
            self.index((x, 1)).unwrap(),
            self.index((x, 2)).unwrap(),
            self.index((x, 3)).unwrap(),
        );
        match self.can_eat(p1, p2, p3, p4) {
            Some(py) => {
                if py != y {
                    res.push((x, py));
                }
            }
            None => (),
        };

        let (p1, p2, p3, p4) = (
            self.index((0, y)).unwrap(),
            self.index((1, y)).unwrap(),
            self.index((2, y)).unwrap(),
            self.index((3, y)).unwrap(),
        );
        match self.can_eat(p1, p2, p3, p4) {
            Some(px) => {
                if px != x {
                    res.push((px, y));
                }
            }
            None => (),
        };
        return res;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat_chess() {
        fn apply_can_eat(x1: i32, x2: i32, x3: i32, x4: i32) -> Option<i32> {
            fn m(x: i32) -> Option<PieceColor> {
                match x {
                    0 => None,
                    1 => Some(PieceColor::Black),
                    2 => Some(PieceColor::White),
                    _ => panic!("unknown num"),
                }
            }
            Board::new().can_eat(m(x1), m(x2), m(x3), m(x4))
        }
        assert_eq!(apply_can_eat(0, 1, 2, 2), Some(1));
        assert_eq!(apply_can_eat(1, 2, 2, 2), Some(0));
        assert_eq!(apply_can_eat(0, 2, 2, 1), Some(3));
        assert_eq!(apply_can_eat(1, 1, 2, 2), None);
        assert_eq!(apply_can_eat(1, 0, 2, 2), None);
        assert_eq!(apply_can_eat(1, 1, 2, 2), None);
        assert_eq!(apply_can_eat(1, 2, 2, 1), None);
        assert_eq!(apply_can_eat(1, 2, 2, 0), Some(0));
        assert_eq!(apply_can_eat(1, 1, 2, 0), Some(2));
    }
}
