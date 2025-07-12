use crate::lib::{line::Line, point::Point};
use std::fs::read_to_string;

pub fn read_polygon_from_file(path: &str) -> Vec<Point> {
    let rows = read_file_rows(path);

    let mut points: Vec<Point> = Vec::new();
    for row in rows {
        points.push(string_to_point(row))
    }
    if points.first().unwrap() != points.last().unwrap() {
        points.push(points.first().unwrap().clone());
    }

    return points;
}

pub fn read_lines_from_file(path: &str) -> Vec<Line> {
    let rows = read_file_rows(path);

    let mut lines: Vec<Line> = Vec::new();
    for row in rows {
        lines.push(string_to_line(row))
    }
    return lines;
}

fn string_to_point(string: String) -> Point {
    let numbers: Vec<f64> = string
        .split(&" ")
        .filter(|s| !s.is_empty())
        .map(|word| word.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    return Point {
        x: numbers[0],
        y: numbers[1],
    };
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
