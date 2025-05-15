use std::collections::HashMap;
use std::fs::read_to_string;

use cg_ss_25::lib::point::Point;

fn read_file_rows(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for row in read_to_string(filename).unwrap().lines() {
        result.push(row.to_string())
    }

    result
}

fn calculate_area_polygon(points: &Vec<Point>) -> f64 {
    let mut area: f64 = 0.0;
    let point_zero = Point { x: 0.0, y: 0.0 };
    for n in 0..points.len() - 1 {
        area += calculate_area_triangle(&point_zero, &points[n], &points[n + 1]);
    }
    area
}

fn calculate_area_triangle(point_0: &Point, point_1: &Point, point_2: &Point) -> f64 {
    let area: f64 = point_0.y * (point_2.x - point_1.x) / 2.0
        + point_1.y * (point_0.x - point_2.x) / 2.0
        + point_2.y * (point_1.x - point_0.x) / 2.0;
    area
}

fn main() {
    let mut states: HashMap<String, Vec<Vec<Point>>> = HashMap::new();
    let mut cities: HashMap<String, Point> = HashMap::new();
    let mut current_id = String::new();
    let path = "../data/02/DeutschlandMitStaedten.svg";
    let data = read_file_rows(path);
    for mut line in data {
        line = line.trim().to_string();
        if line.contains("id=") && !line.contains("svg") {
            current_id = line
                .split("id=")
                .nth(1)
                .unwrap()
                .split(" ")
                .nth(0)
                .unwrap()
                .replace("\"", "")
                .parse()
                .unwrap();
            if line.starts_with("id=") {
                cities.insert(current_id.clone(), Point { x: 0.0, y: 0.0 });
            } else {
                states.insert(current_id.clone(), Vec::new());
            }
        }
        if line.starts_with("M") {
            states.get_mut(&current_id).unwrap().push(Vec::new());
            let coords = line[1..].split(",").collect::<Vec<&str>>();
            let point = Point {
                x: coords[0].parse().unwrap(),
                y: coords[1].parse().unwrap(),
            };
            if let Some(state_vec) = states.get_mut(&current_id) {
                if let Some(last_vec) = state_vec.last_mut() {
                    last_vec.push(point);
                }
            }
        }
        if line.starts_with("l") {
            let coords = line[1..].split(",").collect::<Vec<&str>>();
            let last_point = states
                .get(&current_id)
                .unwrap()
                .last()
                .unwrap()
                .last()
                .unwrap();
            let point = Point {
                x: last_point.x + coords[0].parse::<f64>().unwrap(),
                y: last_point.y + coords[1].parse::<f64>().unwrap(),
            };
            if let Some(state_vec) = states.get_mut(&current_id) {
                if let Some(last_vec) = state_vec.last_mut() {
                    last_vec.push(point);
                }
            }
        }
        if line.starts_with("L") {
            let coords = line[1..].split(",").collect::<Vec<&str>>();
            let point = Point {
                x: coords[0].parse().unwrap(),
                y: coords[1].parse().unwrap(),
            };
            if let Some(last_vec) = states.get_mut(&current_id) {
                if let Some(last_vec) = last_vec.last_mut() {
                    last_vec.push(point);
                }
            }
        }
        if line.starts_with("H") {
            let coords = line[1..].split(",").collect::<Vec<&str>>();
            let last_point = states
                .get(&current_id)
                .unwrap()
                .last()
                .unwrap()
                .last()
                .unwrap();
            let point = Point {
                x: coords[0].parse().unwrap(),
                y: last_point.y,
            };
            if let Some(last_vec) = states.get_mut(&current_id) {
                if let Some(last_vec) = last_vec.last_mut() {
                    last_vec.push(point);
                }
            }
        }
        if line.starts_with("sodipodi:cx") {
            if let Some(point) = cities.get_mut(&current_id) {
                point.x = line
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .replace("\"", "")
                    .parse()
                    .unwrap();
            }
        }
        if line.starts_with("sodipodi:cy") {
            if let Some(point) = cities.get_mut(&current_id) {
                point.y = line
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .replace("\"", "")
                    .parse()
                    .unwrap();
            }
        }
    }

    for (state, vec) in &states {
        let mut area = 0.0;
        for (index, points) in vec.iter().enumerate() {
            let area_polygon = calculate_area_polygon(&points);
            let is_in_polygon;
            if index > 0 {
                is_in_polygon = points[0].is_in_polygon(vec.get(0).unwrap());
                if is_in_polygon {
                    area -= area_polygon.abs();
                } else {
                    area += area_polygon.abs();
                }
            } else {
                area += area_polygon.abs();
            }
            // println!("{} ({})", area_polygon, is_in_polygon);
        }
        println!("{} ({:?})", state, area);
    }

    println!();

    for (city, point) in &cities {
        let mut state_of_city = String::new();
        let mut possible_states: Vec<String> = Vec::new();
        for (state, vec) in &states {
            for (_, points) in vec.iter().enumerate() {
                if point.is_in_polygon(points) {
                    possible_states.push(state.clone());
                }
            }
        }
        let mut state_occurrences: HashMap<String, usize> = HashMap::new();
        for state in possible_states.clone() {
            *state_occurrences.entry(state.clone()).or_insert(0) += 1;
        }
        if state_occurrences.len() == 1 {
            state_of_city = possible_states[0].clone();
        } else {
            for (state, value) in state_occurrences {
                if value == 1 {
                    state_of_city = state.clone();
                }
            }
        }
        println!("{} ({})", city, state_of_city);
    }
}
