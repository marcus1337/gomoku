
//use super::board::tile;
use super::board::Board;
//use super::board::GameResult;
//use super::board::line;
//use super::board::tile::Brick;
use super::board::tile::Point;
use super::board::line::Line;
use super::board::tile::Tile;
use rand::seq::SliceRandom;

fn get_growable_lines(board: Board) -> Vec<Line> {
    let mut lines = Vec::<Line>::new();
    let next_brick = board.get_next_brick();
    let growable_line_1 = [Tile::Empty, Tile::Brick(next_brick)];
    let growable_line_2 = [Tile::Brick(next_brick), Tile::Empty];
    for line in Line::get_lines(2) {
        let tile_line: [Tile; 2] = board.get_tiles(&line).try_into().unwrap();
        if tile_line == growable_line_1 || tile_line == growable_line_2  {
            lines.push(line.clone());
        }
    }
    lines
}

fn get_any_empty_point(board: Board, lines: &Vec<Line>) -> Point {
    let line = lines.choose(&mut rand::thread_rng()).unwrap().clone();
    for point in line.points {
        if !board.has_brick(point) {
            return point;
        }
    }
    return Point{col:-1, row:-1};
}

fn get_any_placement_point(board: Board) -> Point {

    let mid_point = Point{col:7, row:7};
    if !board.has_brick(mid_point) {
        return mid_point;
    }

    let growable_lines = get_growable_lines(board);
    if growable_lines.len() > 0 {
        return get_any_empty_point(board, &growable_lines);
    }

    let empty_points = board.get_empty_tile_points();
    empty_points.choose(&mut rand::thread_rng()).unwrap().clone()
}

fn get_win_lines(board: Board) -> Vec<Line> {
    let mut lines = Vec::<Line>::new();
    let next_brick = board.get_next_brick();
    for line in board.get_potential_winning_lines(next_brick) {
        let tiles = board.get_tiles(&line);
        if tiles.iter().filter(|tile| match tile { Tile::Brick(brick) => brick == &next_brick, _ => false,} ).count() == 4 {
            lines.push(line);
        }
    }
    lines
}


pub fn get_placement_point(board: Board) -> Point {

    let win_lines = get_win_lines(board);
    if win_lines.len() > 0 {
        return get_any_empty_point(board, &win_lines);
    }

    let next_brick = board.get_next_brick();
    let potential_lose_lines = board.get_potential_winning_lines(next_brick.get_opposite());
    if potential_lose_lines.len() > 0 {
        return get_any_empty_point(board, &potential_lose_lines);
    }

    let potential_win_lines = board.get_potential_winning_lines(next_brick);
    if potential_win_lines.len() > 0 {
        return get_any_empty_point(board, &potential_win_lines);
    }
    

    get_any_placement_point(board)
}
