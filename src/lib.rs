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
    pub extern "C" fn gomoku_make() -> Self {
        Self {
            board: Board::new(),
        }
    }

    #[no_mangle]
    pub extern "C" fn gomoku_reset(&mut self) {
        self.board.reset();
    }

    #[no_mangle]
    pub extern "C" fn gomoku_get_ai_move(&self) -> Point {
        ai::get_placement_point(self.board)
    }

    #[no_mangle]
    pub extern "C" fn gomoku_print(&mut self) {
        println!("{}", self.board);
    }

    #[no_mangle]
    pub extern "C" fn gomoku_get_result(&self) -> GameResult {
        return self.board.get_result();
    }

    #[no_mangle]
    pub extern "C" fn gomoku_has_brick(&self, point: Point) -> bool {
        self.board.has_brick(point)
    }
    #[no_mangle]
    pub extern "C" fn gomoku_get_brick(&self, point: Point) -> Brick {
        self.board.get_brick(point)
    }

    #[no_mangle]
    pub extern "C" fn gomoku_get_num_bricks(&self) -> i32 {
        self.board.get_num_bricks()
    }

    #[no_mangle]
    pub extern "C" fn gomoku_get_next_brick(&self) -> Brick {
        self.board.get_next_brick()
    }

    #[no_mangle]
    pub extern "C" fn gomoku_place_brick(&mut self, point: Point) {
        self.board.place_brick(point);
    }

    #[no_mangle]
    pub extern "C" fn gomoku_is_started(&self) -> bool {
        self.board.has_any_bricks()
    }

}
