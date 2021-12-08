use std::fs;

struct Canvas {
    surface: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize
}

#[derive(Debug)]
struct Line {
    u: Point,
    v: Point,
}

impl Canvas {
    fn new(size: usize) -> Self {
        Canvas {
            surface: vec![vec![0; size]; size],
        }
    }
    // fn add_line(&mut self, line: &Line) {
        
    // }
}

impl Line {
    fn from_str(s: &str) -> Self {
        let uv: Vec<&str> = s.split(" -> ").collect();
        let u: Vec<isize> = uv[0]
            .split(",")
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        let v: Vec<isize> = uv[1]
            .split(",")
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        Line {
            u: Point {x: u[0], y: u[1]},
            v: Point {x: v[0], y: v[1]},
        }
    }
    fn slope(&self) -> Option<isize> {
        if self.is_vertical() {
            return None
        }
        let slope = (self.v.y - self.u.y) / (self.v.x - self.u.x);
        Some(slope)
    }
    fn is_vertical(&self) -> bool {
        if self.u.x == self.v.x {
            return true;
        }
        false
    }
    fn is_horizontal(&self) -> bool {
        if self.u.y == self.v.y {
            return true;
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}

mod test {

    use crate::*;

    #[test]
    fn test_part_one() {
        let file_contents = fs::read_to_string("test_input.txt").unwrap();
        let lines: Vec<Line> = file_contents
            .lines()
            .map(|s| Line::from_str(s))
            .filter(|l| l.is_horizontal() || l.is_vertical())
            .collect();
        for l in lines {
            println!("{:?}", l);
            println!("{:?}", l.slope());
        }
    }

    #[test]
    fn test_part_two() {}
}
