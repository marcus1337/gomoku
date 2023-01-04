
use super::tile;
use tile::Point;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub points: Vec<Point>,
}

impl Line {

    fn new(left_point: Point, right_point: Point) -> Self {
        let mut points = Vec::<Point>::new();
        points.push(left_point.clone());
        let mut step_point = left_point.clone();
        while step_point != right_point {
            step_point.step_towards(right_point);
            points.push(step_point.clone());
        }

        assert!(points.len() >= 2 && points.len() <= 5); 
        Self {
            points: points
        }
    }

    fn get_vertical_lines(size: usize) -> Vec<Line>{
        let mut lines = Vec::<Line>::new();


        for col in 0..15 {
            for down_row in 0..(15-size+1) {
                let left_point = Point{col: col as i32, row: (down_row + size - 1) as i32};
                let right_point = Point{col: col as i32, row: down_row as i32 };
                lines.push(Line::new(left_point, right_point));
            }
        }
        
        lines
    }

    fn get_horizontal_lines(size: usize) -> Vec<Line>{
        let mut lines = Vec::<Line>::new();

        for left_col in 0..(15-size+1) {
            for row in 0..15 {
                let left_point = Point{col: left_col as i32, row: row};
                let right_point = Point{col: (left_col + size - 1) as i32, row};
                lines.push(Line::new(left_point, right_point));
            }
        }
        
        lines
    }

    fn get_diagonal_lines(size: usize) -> Vec<Line>{
        let mut lines = Vec::<Line>::new();

        for left_col in 0..(15-size+1) {
            for down_row in 0..(15-size+1) {
                let diagonal_descending_left_point = Point{col: left_col as i32, row: (down_row + size - 1) as i32};
                let diagonal_descending_right_point = Point{col: (left_col + size - 1) as i32, row: down_row as i32 };
                lines.push(Line::new(diagonal_descending_left_point, diagonal_descending_right_point));
                let diagonal_ascending_left_point = Point{col: left_col as i32, row: down_row as i32};
                let diagonal_ascending_right_point = Point{col: (left_col + size - 1) as i32, row: (down_row + size - 1) as i32 };
                lines.push(Line::new(diagonal_ascending_left_point, diagonal_ascending_right_point));
            }
        }
        
        lines
    }

    pub fn get_lines(size: usize) -> Vec<Line>{
        let mut lines = Vec::<Line>::new();
        lines.extend(Line::get_vertical_lines(size));
        lines.extend(Line::get_horizontal_lines(size));
        lines.extend(Line::get_diagonal_lines(size));
        lines
    }

    pub fn get_next_left_point(&self) -> Point {
        self.points[0] - (self.points[1] - self.points[0])
    }
    pub fn has_next_left_point(&self) -> bool {
        self.get_next_left_point().in_bounds()
    }
    pub fn get_next_right_point(&self) -> Point {
        let len = self.points.len();
        self.points[len-1] + (self.points[len-1] - self.points[len-2])
    }
    pub fn has_next_right_point(&self) -> bool {
        self.get_next_right_point().in_bounds()
    }

}

