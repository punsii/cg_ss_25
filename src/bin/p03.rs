use cg_ss_25::lib::data::read_lines_from_file;

// fn calculate_area_polygon(points: &Vec<Point>) -> f64 {
//     let mut area: f64 = 0.0;
//     let point_zero = Point { x: 0.0, y: 0.0 };
//     for n in 0..points.len() - 1 {
//         area += calculate_area_triangle(&point_zero, &points[n], &points[n + 1]);
//     }
//     area
// }
//
// fn calculate_area_triangle(point_0: &Point, point_1: &Point, point_2: &Point) -> f64 {
//     let area: f64 = point_0.y * (point_2.x - point_1.x) / 2.0
//         + point_1.y * (point_0.x - point_2.x) / 2.0
//         + point_2.y * (point_1.x - point_0.x) / 2.0;
//     area
// }

fn main() {
    let lines = read_lines_from_file("../data/01/s_1000_1.dat");
    println!("{:?}", lines[0].p1.x);
}
