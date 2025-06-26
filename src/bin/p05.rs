// use std::{ process::{Command, Stdio}, str };

use std::error::Error;

use good_lp::{Solution, SolverModel, constraint, default_solver, variables};

fn main() -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
