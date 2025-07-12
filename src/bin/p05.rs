use cg_ss_25::lib::{data::read_polygon_from_file, line::Line, point::Point};

use std::error::Error;

use good_lp::{Solution, SolverModel, constraint, default_solver, variables};

fn main() -> Result<(), Box<dyn Error>> {
    let points = read_polygon_from_file("../data/05/testpolygon.txt");

    for i in 0..points.len() - 2 {
        let p1 = points[i + 0].clone();
        let p2 = points[i + 1].clone();
        let p3 = points[i + 2].clone();

        let m = Point {
            x: (p1.x + p2.x + p3.x) / 3.0,
            y: (p1.y + p2.y + p3.y) / 3.0,
        };
        let r2 = 0.0;

        println!("{:?}, {:?},{:?}", p1, p2, p3);

        let l12 = Line::new(p1.clone(), p2.clone());
        let l23 = Line::new(p2.clone(), p3.clone());
        let l31 = Line::new(p3.clone(), p1.clone());

        // variables! {
        //     vars:
        //            a <= 1;
        //       2 <= b <= 4;
        // } // variables can also be added dynamically with ProblemVariables::add
        // let solution = vars
        //     .maximise(10 * (a - b / 5) - b)
        //     .using(default_solver) // IBM's coin_cbc by default
        //     .with(constraint!(a + 2 <= b))
        //     .with(constraint!(1 + a >= 4 - b)) // .with_all(iter) is also available
        //     .solve()?;
        // println!("a={}   b={}", solution.value(a), solution.value(b));
        // println!("a + b = {}", solution.eval(a + b));
        variables! {
            vars:
                   a <= 1;
              2 <= b <= 4;
        } // variables can also be added dynamically with ProblemVariables::add
        let solution = vars
            .maximise(10 * (a - b / 5) - b)
            .using(default_solver) // IBM's coin_cbc by default
            .with(constraint!(a + 2 <= b))
            .with(constraint!(1 + a >= 4 - b)) // .with_all(iter) is also available
            .solve()?;
        println!("a={}   b={}", solution.value(a), solution.value(b));
        println!("a + b = {}", solution.eval(a + b));
    }
    Ok(())
}
