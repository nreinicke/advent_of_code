use std::cmp;
use std::fs;

struct Canvas {
    surface: Vec<Vec<usize>>,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Pixel {
    i: usize,
    j: usize,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    u: Point,
    v: Point,
}

impl Canvas {
    fn new(size: usize) -> Self {
        Canvas {
            surface: vec![vec![0; size]; size],
            size: size,
        }
    }
    fn add_line(&mut self, line: &Line) {
        let pixels = line.get_pixels();
        for p in pixels {
            self.surface[p.i][p.j] += 1;
        }
    }
    fn get_intersections(&self, n: usize) -> usize {
        let mut count: usize = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                if self.surface[i][j] >= n {
                    count += 1;
                }
            }
        }
        count
    }
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
            u: Point { x: u[0], y: u[1] },
            v: Point { x: v[0], y: v[1] },
        }
    }

    fn get_pixels(&self) -> Vec<Pixel> {
        let mut pixels: Vec<Pixel> = Vec::new();
        if self.is_vertical() {
            let start_y = cmp::min(self.u.y, self.v.y);
            let end_y = cmp::max(self.u.y, self.v.y) + 1;
            for y in start_y..end_y {
                pixels.push(Pixel {
                    i: self.u.x as usize,
                    j: y as usize,
                });
            }
        } else {
            let start_x = cmp::min(self.u.x, self.v.x);
            let end_x = cmp::max(self.u.x, self.v.x) + 1;
            for x in start_x..end_x {
                pixels.push(Pixel {
                    i: x as usize,
                    j: self.y(x) as usize,
                });
            }
        }
        pixels
    }

    fn y(&self, x: isize) -> isize {
        let delta_y = (x - self.u.x) * self.slope();
        self.u.y + delta_y
    }

    fn slope(&self) -> isize {
        if self.is_vertical() {
            return isize::MAX;
        }
        let slope = (self.v.y - self.u.y) / (self.v.x - self.u.x);
        slope
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
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<Line> = file_contents.lines().map(|s| Line::from_str(s)).collect();
    let hv_lines: Vec<Line> = lines
        .clone()
        .into_iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .collect();

    let mut canvas = Canvas::new(1000);
    for l in &hv_lines {
        canvas.add_line(l);
    }
    let intersections = canvas.get_intersections(2);
    println!("Part 1 intersections: {}", intersections);

    let mut canvas = Canvas::new(1000);
    for l in &lines {
        canvas.add_line(l);
    }
    let intersections = canvas.get_intersections(2);
    println!("Part 2 intersections: {}", intersections);
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
        let mut canvas = Canvas::new(10);
        for l in lines {
            canvas.add_line(&l);
        }
        let intersections = canvas.get_intersections(2);
        assert_eq!(intersections, 5);
    }

    #[test]
    fn test_part_two() {
        let file_contents = fs::read_to_string("test_input.txt").unwrap();
        let lines: Vec<Line> = file_contents.lines().map(|s| Line::from_str(s)).collect();
        let mut canvas = Canvas::new(10);
        for l in lines {
            canvas.add_line(&l);
        }
        let intersections = canvas.get_intersections(2);
        assert_eq!(intersections, 12);
    }
}
