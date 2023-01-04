
pub mod tile;
pub mod line;

use std::fmt;
use tile::Point;
use tile::Tile;
use line::Line;
use tile::Brick;


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    OneWin,
    TwoWin,
    Draw,
    OnGoing,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Board{
    tiles: [[Tile; 15]; 15],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Empty; 15]; 15],
        }
    }

    pub fn reset(&mut self) {
        *self = Board::new();
    }

    fn get_tile(&self, point: Point) -> Tile {
        self.tiles[point.col as usize][point.row as usize].clone()
    }

    fn is_win_line(&self, line: &Line) -> bool {
        let first_tile = self.get_tile(line.points[0]);
        let all_same = line.points.iter().all(|point| self.get_tile(*point) == self.get_tile(line.points[0]));
        line.len() == 5 && all_same && first_tile != Tile::Empty
    }

    pub fn get_win_line(&self) -> [Point; 5] {
        let points = [Point{col:-1,row:-1}; 5];
        for line in Line::get_lines(5) {
            if self.is_win_line(&line) {
                return line.points.as_slice().try_into().unwrap()
            }
        }
        points
    }

    fn has_win_line(&self) -> bool {
        let win_line = self.get_win_line();
        win_line[0].col != -1
    }

    fn get_winner(&self, win_line: [Point; 5]) -> GameResult {
        let tile = self.get_tile(win_line[0]);
        if tile == Tile::Brick(Brick::One){
            return GameResult::OneWin;
        } else {
            return GameResult::TwoWin;
        }
    }

    fn can_place_any(&self) -> bool {
        self.tiles.iter().any(|row| row.iter().any(|&tile| tile == Tile::Empty)) 
    }

    pub fn get_result(&self) -> GameResult {
        if self.has_win_line() {
            return self.get_winner(self.get_win_line());
        }
        if self.can_place_any() {
            return GameResult::OnGoing;
        }
        GameResult::Draw
    }

    pub fn has_brick(&self, point: Point) -> bool {
        self.get_tile(point) != Tile::Empty
    }

    pub fn get_brick(&self, point: Point) -> Brick {
        let brick = match self.get_tile(point){
            Tile::Brick(brick) => brick,
            _ => panic!("Invalid value"),
        };
        brick
    }

    pub fn get_num_bricks(&self) -> i32 {
        let count_bricks = self
            .tiles
            .iter()
            .flatten()
            .filter(|x| match x {
                Tile::Brick(_) => true,
                _ => false,
            })
            .count();
        count_bricks as i32
    }

    pub fn get_next_brick(&self) -> Brick {
        if self.get_num_bricks() % 2 == 0 {
            return Brick::One;
        } else {
            return Brick::Two;
        };
    }

    pub fn place_brick(&mut self, point: Point){
        self.tiles[point.col as usize][point.row as usize] = Tile::Brick(self.get_next_brick());
    }

}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = "".to_owned();
        for row in 0..15 {
            for col in 0..15 {
                board_string += self.tiles[col][15 - row].to_string().as_str();
            }
            board_string += "\n";
        }
        write!(f, "{}", board_string)
    }
}


