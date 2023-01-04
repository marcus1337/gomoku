
use super::tile;
use tile::Point;

pub struct Line {
    pub points: Vec<Point>,
}

impl Line {

    pub fn new(left_point: Point, right_point: Point) -> Self {
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