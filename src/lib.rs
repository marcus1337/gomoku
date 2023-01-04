extern crate libc;

pub mod board;
pub mod ai;
use self::board::Board;
//use board::line;
use board::tile::Brick;
use board::tile::Point;
use board::GameResult;

//cbindgen --output target/release/gomoku.h

#[repr(C)]
pub struct Gomoku {
    board: Board,
}

impl Gomoku {
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
    pub extern "C" fn get_ai_move(&self) -> Point {
        ai::get_placement_point(self.board)
    }

    #[no_mangle]
    pub extern "C" fn print(&mut self) {
        println!("{}", self.board);
    }

    #[no_mangle]
    pub extern "C" fn get_result(&self) -> GameResult {
        return self.board.get_result();
    }

    #[no_mangle]
    pub extern "C" fn has_brick(&self, point: Point) -> bool {
        self.board.has_brick(point)
    }
    #[no_mangle]
    pub extern "C" fn get_brick(&self, point: Point) -> Brick {
        self.board.get_brick(point)
    }

    #[no_mangle]
    pub extern "C" fn get_num_bricks(&self) -> i32 {
        self.board.get_num_bricks()
    }

    #[no_mangle]
    pub extern "C" fn get_next_brick(&self) -> Brick {
        self.board.get_next_brick()
    }

    #[no_mangle]
    pub extern "C" fn place_brick(&mut self, point: Point) {
        self.board.place_brick(point);
    }

    #[no_mangle]
    pub extern "C" fn is_started(&self) -> bool {
        self.board.has_any_bricks()
    }

}
