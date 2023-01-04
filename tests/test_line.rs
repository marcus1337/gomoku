
#[path = "../src/lib.rs"]
mod lib;
use lib::board::line;
use line::Line;

#[cfg(test)]
mod line_tests {
    use super::*;

    #[test]
    fn make_lines() {
        let lines = Line::get_lines(5);
        assert_eq!(lines.len(), 572);
    }
}
