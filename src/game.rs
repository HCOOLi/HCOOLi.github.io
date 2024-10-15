use std::fmt::{write, Display, Formatter};
use std::slice::ChunksExact;

#[derive(Copy, Clone, PartialEq, Debug)]
enum PlayerColor {
    Black,
    White,
}
impl PlayerColor {
    fn opposite(&self) -> Self {
        match self {
            Black => White,
            White => Black,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl Movement {
    fn number(&self) -> (i8, i8) {
        match self {
            Movement::Left => (-1, 0),
            Right => (1, 0),
            Up => (0, 1),
            Down => (0, -1),
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Movement::Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

impl Display for PlayerColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Black => write!(f, "{}", 1),
            White => write!(f, "{}", 2),
        }
    }
}
struct Table([[Option<PlayerColor>; 4]; 4]);

impl Table {
    pub fn new() -> Self {
        Table([[Some(Black); 4], [None; 4], [None; 4], [Some(White); 4]])
    }
    fn index(&self, (x, y): (i8, i8)) -> Result<Option<PlayerColor>, String> {
        let err1 = Err(String::from("out of range"));
        if x < 0 || x >= 4 || y < 0 || y >= 4 {
            return err1;
        }
        let (x, y) = (x as usize, y as usize);
        return Ok(self.0[x][y]);
    }

    fn set(&mut self, (x, y): (i8, i8), c: Option<PlayerColor>) -> Result<(), String> {
        let err1 = Err(String::from("out of range"));
        if x < 0 || x >= 4 || y < 0 || y >= 4 {
            return err1;
        }
        let (x, y) = (x as usize, y as usize);
        self.0[x][y] = c;
        return Ok(());
    }

    pub fn print(&self) {
        for x in self.0 {
            for y in x {
                print!("{} ", y)
            }
            println!()
        }
    }

    fn move_chess(
        &mut self,
        (from_x, from_y): (i32, i32),
        (to_x, to_y): (i32, i32),
    ) -> Result<(), String> {
        let err2 = Err(String::from("already has one"));
        let err3 = Err(String::from("empty"));
        let current = self.index(x, y)?;
        if current == Empty {
            return err3;
        }
        let (dx, dy) = m.number();
        let next = self.index(x + dx, y + dy)?;
        if next != Empty {
            return err2;
        }
        self.mut_index(x, y, Empty)?;
        self.mut_index(x + dx, y + dy, current)?;
        self.eat_chess(x + dx, y + dy);
        Ok(())
    }
    fn _can_eat_(
        x1: Option<PieceColor>,
        x2: Option<PieceColor>,
        x3: Option<PieceColor>,
        x4: Option<PieceColor>,
    ) -> Option<i32> {
        match (x1, x2, x3, x4) {
            (
                Some(PieceColor::Black),
                Some(PieceColor::Black),
                Some(PieceColor::White),
                None | Some(PieceColor::Black),
            ) => Some(2),
            (
                Some(PieceColor::Black),
                Some(PieceColor::White),
                Some(PieceColor::White),
                None | Some(PieceColor::White),
            ) => Some(0),
        }
    }
    fn can_eat_chess(mut self, x: i8, y: i8) -> bool {
        let current = self.index((x, y)).unwrap();
        let arround = [Left, Right, Up, Down];
        for arr in arround {
            let (dx, dy) = arr.number();
            let next = self.index(x + dx, y + dy);
            println!("next={:?}", next);

            match next {
                Err(_) => continue,
                Ok(Empty) => continue,
                Ok(next_color) => {
                    if next_color == current {
                        let opp_chess = self.index(x - dx, y - dy);
                        println!("opp={:?}", opp_chess);
                        match opp_chess {
                            Err(_) => continue,
                            Ok(next_color) => {
                                if next_color == Empty {
                                    continue;
                                }
                                if next_color == current.opposite() {
                                    let opp_chess_2 = self.index(x - 2 * dx, y - 2 * dy);
                                    match opp_chess_2 {
                                        Err(_) => {
                                            println!("chess {x},{y} eat {},{}", x - dx, y - dy);
                                            self.mut_index(x - dx, y - dy, Empty).unwrap()
                                        }
                                        Ok(next_color) => {
                                            if next_color == Empty {
                                                println!("chess {x},{y} eat {},{}", x - dx, y - dy);
                                                self.mut_index(x - dx, y - dy, Empty).unwrap();
                                            }
                                        }
                                    };
                                }
                            }
                        };

                        let opp_chess = self.index(x + 2 * dx, y + 2 * dy);
                        match opp_chess {
                            Err(_) => continue,
                            Ok(next_color) => {
                                if next_color == Empty {
                                    continue;
                                }
                                if next_color == current.opposite() {
                                    let opp_chess_2 = self.index(x + 3 * dx, y + 3 * dy);
                                    match opp_chess_2 {
                                        Err(_) => {
                                            println!(
                                                "chess {x},{y} eat {},{}",
                                                x + 2 * dx,
                                                y + 2 * dy
                                            );
                                            self.mut_index(x + 2 * dx, y + 2 * dy, Empty).unwrap();
                                        }
                                        Ok(next_color) => {
                                            if next_color == Empty {
                                                println!(
                                                    "chess {x},{y} eat {},{}",
                                                    x + 2 * dx,
                                                    y + 2 * dy
                                                );
                                                self.mut_index(x + 2 * dx, y + 2 * dy, Empty)
                                                    .unwrap();
                                            }
                                        }
                                    };
                                }
                            }
                        };
                    }
                }
            }
        }
        false
    }
}

fn _can_eat_(
    x1: Option<PieceColor>,
    x2: Option<PieceColor>,
    x3: Option<PieceColor>,
    x4: Option<PieceColor>,
) -> Option<i32> {
    match (x1, x2, x3, x4) {
        (
            Some(PieceColor::Black),
            Some(PieceColor::Black),
            Some(PieceColor::White),
            None | Some(PieceColor::Black),
        ) => Some(2),
        (
            Some(PieceColor::Black),
            Some(PieceColor::White),
            Some(PieceColor::White),
            None | Some(PieceColor::White),
        ) => Some(0),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eat_chess() {
        assert_eq!(
            _can_eat_(
            Some(PieceColor::Black),
            Some(PieceColor::Black),
            Some(PieceColor::White),
            Some(PieceColor::Black)),,Some(2));
        assert_eq!(
            _can_eat_(
                Some(PieceColor::Black),
                Some(PieceColor::White),
                Some(PieceColor::White),
                None,
            ),
            Some(0)
        );
    }
}
