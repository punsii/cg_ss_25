use plotters::prelude::*;
use std::{
    process::{Command, Stdio},
    str,
};

const TWO: i32 = 2;
const OUT_PATH: &str = "./plots";
const REPETITIONS: i32 = 5;
const MIN_NUM_POINT_EXPONENT: i32 = 4;
const MAX_NUM_POINT_EXPONENT: i32 = 11;
const DIMENSIONS: &[i32] = &[2, 3, 4, 5, 6];
const DISTRIBUTIONS: &[Distribution] = &[
    Distribution {
        // "c", // add a unit cube to the output ('c G2.0' sets size)
        name: "UnitCube",
        flag: 'c',
    },
    Distribution {
        // "d", // add a unit diamond to the output ('d G2.0' sets size)
        name: "UnitDiamond",
        flag: 'd',
    },
    // Distribution {
    //     //     "r", // generate a regular polygon, ('r s Z1 G0.1' makes a cone)
    //     name: "regularPolygon",
    //     flag: 'r',
    // },
    Distribution {
        //     "s", // generate cospherical points
        name: "Cospherical",
        flag: 's',
    },
    Distribution {
        //     "x", // generate random points in simplex, may use 'r' or 'Wn'
        name: "Simplex",
        flag: 'x',
    },
    Distribution {
        //     "y", // same as 'x', plus simplex
        name: "SimplexSimplex?",
        flag: 'y',
    },
];

#[derive(Debug)]
struct TestResult {
    n_points_power: i32,
    dimension: i32,
    seconds: f64,
}

#[derive(Debug)]
struct Distribution<'a> {
    name: &'a str,
    flag: char,
}

fn point_generator(distribution_flag: char, n_points: i32, dimension: i32) -> std::process::Child {
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

fn measure_qhull_runtime(points_stream: std::process::Child) -> f64 {
    let qhull = Command::new("qhull")
        .stdin(Stdio::from(points_stream.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = qhull.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();

    result
        .split("\n")
        .find(|s| s.contains("CPU seconds to compute hull"))
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .to_string()
        .parse::<f64>()
        .unwrap_or_else(|_| panic!("Could not parse time from output: {:?}", result))
}

fn get_data_point(
    n_point_power: i32,
    dimensions: i32,
    distribution_results: &Vec<TestResult>,
) -> f64 {
    distribution_results
        .iter()
        .filter(|test_result| test_result.n_points_power == n_point_power)
        .filter(|test_result| test_result.dimension == dimensions)
        // map reduce implementation of the average execution time
        .map(|test_result| (1, test_result.seconds))
        .reduce(|accumulator, element| (accumulator.0 + element.0, accumulator.1 + element.1))
        .map(|accumulator| accumulator.1 / accumulator.0 as f64)
        .unwrap()
}

fn save_plot(
    distribution_name: &str,
    distribution_results: Vec<TestResult>,
    n_point_powers: &Vec<i32>,
) {
    let out_file_name = format!("{OUT_PATH}/{distribution_name}.svg");

    let plot = SVGBackend::new(&out_file_name, (1024, 760)).into_drawing_area();
    plot.fill(&WHITE).unwrap();

    let z_max = distribution_results
        .iter()
        .map(|r| r.seconds)
        .fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&plot)
        .caption(distribution_name, ("sans-serif", 20))
        .build_cartesian_3d(
            *n_point_powers.iter().min().unwrap()..*n_point_powers.iter().max().unwrap(), // x = n_points
            (0.0..z_max).step(z_max / 30.0), // y = seconds
            *DIMENSIONS.iter().min().unwrap()..*DIMENSIONS.iter().max().unwrap(), // z = dimensions
        )
        .unwrap();
    chart.with_projection(|mut p| {
        // p.pitch = 1.3;
        p.yaw = 0.5 + 1.0 * 3.14;
        p.scale = 0.8;
        p.into_matrix()
    });

    chart
        .configure_axes()
        .label_style(("Calibri", 15))
        // .light_grid_style(BLACK.mix(0.15))
        // .max_light_lines(3)
        // .axis_panel_style(GREEN.mix(0.1))
        .x_formatter(&|x| format!("x=2^{:?}", x))
        .z_formatter(&|z| format!("y={z}"))
        .y_formatter(&|y| format!("z={:.3}", y))
        .draw()
        .unwrap();

    chart
        .draw_series(
            SurfaceSeries::xoz(
                n_point_powers.iter().copied(),
                DIMENSIONS.iter().copied(),
                |n_point_powers, dimensions| {
                    get_data_point(n_point_powers, dimensions, &distribution_results)
                },
            )
            .style_func(&|&v| (VulcanoHSL::get_color(v / z_max)).into()),
        )
        .unwrap();

    plot.present().unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    plot.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", &out_file_name);
}

fn main() {
    let n_point_powers: Vec<i32> = (MIN_NUM_POINT_EXPONENT..MAX_NUM_POINT_EXPONENT).collect();

    for distribution in DISTRIBUTIONS.iter() {
        let mut distribution_results: Vec<TestResult> = Vec::new();
        println!(
            "Generating points with distribution: {:?}.",
            distribution.name
        );

        for dimension in DIMENSIONS.iter() {
            println!("Calculating hull for {:?} dimensions.", dimension);
            for n_point_power in n_point_powers.clone().into_iter() {
                let n_points = TWO.pow(n_point_power as u32);
                print!("Points: {:?} | ", n_points);
                for _ in 0..REPETITIONS {
                    let points_stream = point_generator(distribution.flag, n_points, *dimension);

                    let time = measure_qhull_runtime(points_stream);
                    print!("{:?}s ", time);

                    distribution_results.push(TestResult {
                        n_points_power: n_point_power,
                        dimension: *dimension,
                        seconds: time,
                    });
                }
                println!();
            }
            print!("\n\n");
        }
        save_plot(distribution.name, distribution_results, &n_point_powers);
        print!("\n\n\n");
    }
}
