use cg_ss_25::lib::{data::read_polygon_from_file, line::Line, point::Point};
use plotters::prelude::*;

use good_lp::{Solution, SolverModel, constraint, default_solver, variables};
use std::error::Error;

// const FILE_NAME: &str = "testpolygon.txt";
const FILE_NAME: &str = "polygon.txt";

const PATH: &str = "../data/05/";

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = PATH.to_string() + FILE_NAME;

    let mut points = read_polygon_from_file(&file_path);

    // Calculate average over all vertices using map reduce
    let center = points
        .iter()
        .skip(1) // skip first element, as it will be identical to the last
        .map(|&point| (1.0, point))
        .reduce(|accumulator, element| (accumulator.0 + element.0, accumulator.1 + element.1))
        .map(|accumulator| Point {
            x: accumulator.1.x / accumulator.0,
            y: accumulator.1.y / accumulator.0,
        })
        .unwrap();

    // moving the points to around the origin
    // (alternative would be to swap normals based on the direction of the mass center)
    points = points.into_iter().map(|p| p - center).collect();

    let lines: Vec<Line> = points
        .windows(2)
        .map(|pair| Line::new(pair[0], pair[1]))
        .collect();

    variables! {
        vars:
            x;
            y;
            r >= 0;
    }

    let mut problem = vars.maximise(r).using(default_solver);

    for line in &lines {
        let n = line.n;

        // Constraint: n · (x, y) + r <= a
        problem = problem.with(constraint!(n.x * x + n.y * y - line.a + r <= 0.0));
    }

    let solution = problem.solve().unwrap();

    let solution_x = solution.value(x);
    let solution_y = solution.value(y);
    let radius = solution.value(r);

    println!(
        "Optimal center: ({:.2}, {:.2})",
        solution_x + center.x,
        solution_y + center.y
    );
    println!("Maximum inscribed radius: {:.2}", radius);

    let circle_center = Point {
        x: solution.value(x),
        y: solution.value(y),
    };

    plot_results(&points, &lines, circle_center, radius)?;

    Ok(())
}

fn plot_results(
    points: &[Point],
    lines: &[Line],
    circle_center: Point,
    radius: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("testpolygon.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Estimate bounding box
    let (range_min, range_max) = points
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), p| {
            (min.min(p.x).min(p.y), max.max(p.x).max(p.y))
        });

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Polygon with Normals", ("sans-serif", 30))
        .set_all_label_area_size(10)
        .build_cartesian_2d(
            range_min - 10.0..range_max + 10.0,
            range_min - 10.0..range_max + 10.0,
        )?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Draw vertices
    chart.draw_series(
        points
            .iter()
            .skip(1)
            .map(|p| Circle::new((p.x, p.y), 5, RED.filled())),
    )?;

    // Draw edges
    chart.draw_series(lines.iter().map(|line| {
        PathElement::new(
            vec![(line.p1.x, line.p1.y), (line.p2.x, line.p2.y)],
            BLACK.stroke_width(2),
        )
    }))?;

    // Compute average line length
    let avg_length = lines.iter().map(|line| line.length()).sum::<f64>() / lines.len() as f64;
    let normal_length = avg_length * 0.4;
    let normal_style = BLUE.stroke_width(2);

    // Draw normals
    for line in lines {
        let mid = Point {
            x: (line.p1.x + line.p2.x) / 2.0,
            y: (line.p1.y + line.p2.y) / 2.0,
        };
        let dir = Point {
            x: line.n.x * normal_length,
            y: line.n.y * normal_length,
        };
        let end = Point {
            x: mid.x + dir.x,
            y: mid.y + dir.y,
        };

        // Line for normal + Arrowhead
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(mid.x, mid.y), (end.x, end.y)],
            normal_style,
        )))?;
        let dx = dir.x;
        let dy = dir.y;
        let norm = (dx * dx + dy * dy).sqrt();
        if norm > 0.0 {
            let ux = dx / norm;
            let uy = dy / norm;
            let head_size = normal_length * 0.2;
            let left = Point {
                x: end.x - head_size * (ux + uy),
                y: end.y - head_size * (uy - ux),
            };
            let right = Point {
                x: end.x - head_size * (ux - uy),
                y: end.y - head_size * (uy + ux),
            };
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(left.x, left.y), (end.x, end.y), (right.x, right.y)],
                normal_style,
            )))?;
        }
    }

    let circle_path: Vec<_> = (0..360)
        .step_by(10)
        .map(|angle| {
            let rad = (angle as f64).to_radians();
            (
                circle_center.x + radius * rad.cos(),
                circle_center.y + radius * rad.sin(),
            )
        })
        .collect();

    chart.draw_series(std::iter::once(PathElement::new(
        circle_path,
        RED.stroke_width(2),
    )))?;

    // // Draw center point and circle. (Getting this size correct was a pita....)
    // chart.draw_series(std::iter::once(Circle::new(
    //     (circle_center.x, circle_center.y),
    //     5,
    //     MAGENTA.filled(),
    // )))?;
    // let (plot_width, plot_height) = chart.plotting_area().dim_in_pixel();
    // let x_range = chart.plotting_area().get_x_range();
    // let y_range = chart.plotting_area().get_y_range();
    // let x_len = x_range.end - x_range.start;
    // let y_len = y_range.end - y_range.start;
    // let x_pixels_per_unit = plot_width as f64 / x_len;
    // let y_pixels_per_unit = plot_height as f64 / y_len;
    // let pixels_per_unit = (x_pixels_per_unit + y_pixels_per_unit) / 2.0;
    // let r_pixel = (radius * pixels_per_unit).round() as i32;
    // chart.draw_series(std::iter::once(Circle::new(
    //     (circle_center.x, circle_center.y),
    //     r_pixel, // <- properly scaled pixel radius
    //     MAGENTA.stroke_width(2),
    // )))?;

    Ok(())
}
