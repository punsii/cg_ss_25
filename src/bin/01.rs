use std::fs::read_to_string;

// use core::f64::EPSILON; // => ~2.3E-16
const EPSILON: f64 = 1E-12;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn ccw(p: &Point, q: &Point, r: &Point) -> i32 {
    let ccw = p.x * q.y - p.y * q.x + q.x * r.y - q.y * r.x + p.y * r.x - p.x * r.y;

    if ccw < -EPSILON {
        -1
    } else if ccw < EPSILON {
        0
    } else {
        1
    }
}

impl Point {}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
    pub n: Option<Point>,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Line {
        let x = p2.x - p2.y;
        let y = p1.y - p1.x;

        let a = p1.y * p2.x - p1.x * p2.y;
        let n = if a > 0.0 {
            Point { x, y }
        } else {
            Point { x: -x, y: -y }
        };

        Line { p1, p2, n: Some(n) }
    }

    fn crosses(&self, other: &Line) -> bool {
        let a = ccw(&other.p1, &other.p2, &self.p1);
        let b = ccw(&other.p1, &other.p2, &self.p2);
        let c = ccw(&self.p1, &self.p2, &other.p1);
        let d = ccw(&self.p1, &self.p2, &other.p2);

        if a != b && c != d {
            //  This should always be true
            true
        } else {
            //  TODO: this is definitely not always correct!
            false
        }
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
