
pub mod tile;
pub mod line;

//use std::fmt;
//use tile::Point;
use tile::Tile;
//use line::Line;
//use tile::Brick;


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
    pub tiles: [[Tile; 15]; 15],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Empty; 15]; 15],
        }
    }

}