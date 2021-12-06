use std::collections::HashMap;
use bresenham::Bresenham;

mod helpers;

type Point = (isize, isize);

struct Line {
    start: Point,
    end: Point
}

trait GetPoints {
    fn get_all_points_along(&self) -> Vec<Point>;
}

impl GetPoints for Line {
    fn get_all_points_along(&self) -> Vec<Point> {
        let mut line: Vec<Point> = Bresenham::new(self.start, self.end).map(|e| e as Point).collect();
        line.push(self.end as Point); // apparently my cheat here doesnt include the end point so i have to add it
        line
    }
}

trait ChecksForDiagonal {
    fn is_diagonal(&self) -> bool;
}

impl ChecksForDiagonal for Line {
    fn is_diagonal(&self) -> bool {
        self.start.0 != self.end.0 && self.start.1 != self.end.1
    }
}

impl Line {
    fn new(line: &str) -> Self {
        let points: Vec<isize> = line.replace(" -> ", ",").split(",").map(|e| e.parse::<isize>().unwrap()).collect();
        Line { start: (points[0], points[1]), end: (points[2], points[3]) }
    }
}

fn get_overlapping(data: &String, ignore_diagonals: bool) -> usize {
    let lines = data.lines().map(|line| Line::new(line));
    type Ground<'a> = HashMap<Point, usize>;

    let mut ground: Ground = HashMap::new();
    lines.filter(|line| !(line.is_diagonal() && ignore_diagonals)).for_each(|line| {
        for p in line.get_all_points_along() {
            let check_if_exists = ground.insert(p, 1);
            if check_if_exists != None {
                ground.insert(p, 2);
            }
        }
    });
    ground.iter().map(|(_key, &value)| if value > 1 { 1 } else { 0 }).sum()
}
fn main() {
    let data = helpers::get_data_for_day("5").unwrap();
    println!("The answer to Part 1 is {}", get_overlapping(&data, true));
    println!("The answer to Part 2 is {}", get_overlapping(&data, false));

}