
extern crate libc;

pub mod board;
use self::board::Board;
use board::line;
use board::tile::Point;

//cbindgen --output target/release/gomoku.h

#[repr(C)]
pub struct Gomoku {
    pub board: Board,
}

impl Gomoku{

    #[no_mangle]
    pub extern "C" fn make() -> Self {
        Self {
            board: Board::new(),
        }
    }

    #[no_mangle]
    pub extern "C" fn reset(&mut self) {
        self.board.reset();
    }

    #[no_mangle]
    pub extern "C" fn get_ai_move(&mut self) -> Point {
        Point{col:-1,row:-1}
    }

    #[no_mangle]
    pub extern "C" fn print(&mut self) {
        println!("{}", self.board);
    }

}