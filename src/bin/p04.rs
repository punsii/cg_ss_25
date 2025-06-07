use plotters::prelude::*;
use std::{
    process::{Command, Stdio},
    str,
};

const OUT_PATH: &str = "./plots";
const REPETITIONS: u8 = 3;
const N_POINTS: &[u32] = &[10, 100];
const DIMENSIONS: &[u8] = &[2, 3, 4, 5, 6];
const DISTRIBUTIONS: &[Distribution] = &[
    Distribution {
        // "c", // add a unit cube to the output ('c G2.0' sets size)
        name: "UnitCube",
        flag: 'c',
    },
    // Distribution {
    //     // "d", // add a unit diamond to the output ('d G2.0' sets size)
    //     name: "UnitDiamond",
    //     flag: 'd',
    // },
    // Distribution {
    //     //     "l", // generate a regular 3-d spiral
    //     name: "3dSpiral",
    //     flag: 'l',
    // },
    // Distribution {
    //     //     "r", // generate a regular polygon, ('r s Z1 G0.1' makes a cone)
    //     name: "regularPolygon",
    //     flag: 'r',
    // },
    // Distribution {
    //     //     "s", // generate cospherical points
    //     name: "Cospherical",
    //     flag: 's',
    // },
    // Distribution {
    //     //     "x", // generate random points in simplex, may use 'r' or 'Wn'
    //     name: "Simplex",
    //     flag: 'x',
    // },
    // Distribution {
    //     //     "y", // same as 'x', plus simplex
    //     name: "SimplexSimplex?",
    //     flag: 'y',
    // },
];

#[derive(Debug)]
struct TestResult {
    n_points: u32,
    dimension: u8,
    seconds: f32,
}

#[derive(Debug)]
struct Distribution<'a> {
    name: &'a str,
    flag: char,
}

fn point_generator(distribution_flag: char, n_points: &u32, dimension: &u8) -> std::process::Child {
    // generate random points
    let points_stream = Command::new("rbox")
        .arg(n_points.to_string())
        .arg(format!("D{}", dimension))
        .arg(distribution_flag.to_string())
        .stdout(Stdio::piped())
        .spawn()
        .expect("rbox command failed to start");
    points_stream
}

fn measure_qhull_runtime(points_stream: std::process::Child) -> f32 {
    let qhull = Command::new("qhull")
        .stdin(Stdio::from(points_stream.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = qhull.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();

    return result
        .split("\n")
        .filter(|s| !s.is_empty())
        .last()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .to_string()
        .parse::<f32>()
        .unwrap();
}

fn get_data_point(n_points: u32, dimensions: u8, distribution_results: &Vec<TestResult>) -> f32 {
    distribution_results
        .into_iter()
        .filter(|test_result| test_result.n_points == n_points)
        .filter(|test_result| test_result.dimension == dimensions)
        // map reduce implementation of the average execution time
        .map(|test_result| (1, test_result.seconds))
        .reduce(|accumulator, element| (accumulator.0 + element.0, accumulator.1 + element.1))
        .map(|accumulator| accumulator.1 / accumulator.0 as f32)
        .unwrap()
}

fn save_plot(distribution_name: &str, distribution_results: Vec<TestResult>) {
    println!("{:?}", get_data_point(10, 3, &distribution_results));
    println!("{:?}", distribution_name);
    println!("{:?}", distribution_results[0].n_points);
    println!("{:?}", distribution_results[0].dimension);
    println!("{:?}", distribution_results[0].seconds);
    println!("{:?}", OUT_PATH);
}

fn main() {
    for distribution in DISTRIBUTIONS.into_iter() {
        let mut distribution_results: Vec<TestResult> = Vec::new();

        for n_points in N_POINTS.into_iter() {
            for dimension in DIMENSIONS.into_iter() {
                for _ in 0..REPETITIONS {
                    let points_stream = point_generator(distribution.flag, n_points, dimension);

                    let time = measure_qhull_runtime(points_stream);
                    // println!("{:?}", time);

                    // print!("\n\n\n\n\n\n\n\n\n");
                    distribution_results.push(TestResult {
                        n_points: *n_points,
                        dimension: *dimension,
                        seconds: time,
                    });
                }
            }
        }
        println!("{:?}", distribution_results);

        save_plot(distribution.name, distribution_results);
    }
}
