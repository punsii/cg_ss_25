use std::fs::read_to_string;

// use core::f64::EPSILON; // => ~2.3E-16
const EPSILON: f64 = 1E-12;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn ccw(&self, line: &Line) -> i32 {
        let p1 = &line.p1;
        let p2 = &line.p2;

        let ccw = p1.y * self.x - p2.y * self.x + p2.x * self.y - p1.x * self.y - p1.y * p2.x
            + p1.x * p2.y;
        if ccw < -EPSILON {
            return -1;
        } else if ccw < EPSILON {
            return 0;
        } else {
            return 1;
        }
    }
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
    pub n: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Line {
        let a = p1.y * p2.x - p1.x * p2.y;
        let n = if a > 0.0 {
            Point {
                x: (p2.x - p2.y),
                y: (p1.y - p1.x),
            }
        } else {
            Point {
                x: -(p2.x - p2.y),
                y: -(p1.y - p1.x),
            }
        };

        Line { p1, p2, n }
    }

    fn crosses(&self, other: &Line) -> bool {
        let a = self.p1.ccw(other);
        let b = self.p2.ccw(other);
        let c = other.p2.ccw(self);
        let d = other.p2.ccw(self);

        a != b && c != d
    }
}

fn string_to_line(string: String) -> Line {
    let numbers: Vec<f64> = string
        .split(&" ")
        .map(|word| word.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    Line::new(
        Point {
            x: numbers[0],
            y: numbers[1],
        },
        Point {
            x: numbers[2],
            y: numbers[3],
        },
    )
}

fn read_file_rows(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for row in read_to_string(filename).unwrap().lines() {
        result.push(row.to_string())
    }

    result
}

fn main() {
    let rows = read_file_rows("../data/01/s_1000_1.dat");

    let mut lines: Vec<Line> = Vec::new();
    for row in rows {
        lines.push(string_to_line(row))
    }

    let test_line = Line::new(
        Point {
            x: -100.0,
            y: -100.0,
        },
        Point { x: 100.0, y: 100.0 },
    );
    for line in &lines[0..10] {
        print!(
            "({}, {}),  ({}, {})",
            line.p1.x, line.p1.y, line.p2.x, line.p2.y
        );

        print!(" {:?} ", line.crosses(&test_line));

        print!(
            "({}, {}),  ({}, {})\n\n",
            test_line.p1.x, test_line.p1.y, test_line.p2.x, test_line.p2.y
        );
    }
}
