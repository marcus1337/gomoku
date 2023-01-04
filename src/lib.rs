
extern crate libc;
pub mod board;
use self::board::Board;

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

}