use std::fs;

struct Plane {
    plane: Vec<Vec<usize>>,
}
#[derive(Debug)]
struct Line {
    u: [usize; 2],
    v: [usize; 2],
}

impl Plane {
    fn new(size: usize) -> Self {
        Plane {
            plane: vec![vec![0; size]; size],
        }
    }
}

impl Line {
    fn from_str(s: &str) -> Self {
        let uv: Vec<&str> = s.split(" -> ").collect();
        let u: Vec<usize> = uv[0]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let v: Vec<usize> = uv[1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Line {
            u: [u[0], u[1]],
            v: [v[0], v[1]],
        }
    }
    fn is_horizontal(&self) -> bool {
        if self.u[0] == self.v[0] {
            return true;
        }
        false
    }
    fn is_vertical(&self) -> bool {
        if self.u[1] == self.v[1] {
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
        println!("{:?}", lines);
    }

    #[test]
    fn test_part_two() {}
}
