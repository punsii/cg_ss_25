use std::fs::read_to_string;
use std::time::SystemTime;

mod p01_unique_cases;

// use core::f64::EPSILON; // => ~2.3E-16
const EPSILON: f64 = 1E-12;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn is_in_boundary(&self, line: &Line) -> bool {
        let xmin = f64::min(line.p1.x, line.p2.x);
        let xmax = f64::max(line.p1.x, line.p2.x);
        let ymin = f64::min(line.p1.y, line.p2.y);
        let ymax = f64::max(line.p1.y, line.p2.y);

        self.x >= xmin && self.x <= xmax && self.y >= ymin && self.y <= ymax
    }
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

        let h1 = a * b;
        let h2 = c * d;
        // a is on the same side as b <=> h = 1
        if h1 == 1 || h2 == 1 {
            // at least one line is completely on one side of the other line
            return false;
        }
        // a and b are on different sides <=> h = -1
        if h1 == -1 && h2 == -1 {
            // both lines are on both sides of the other line
            return true;
        }

        // from here on every possibility has *at least* one point that is 'inline'
        // if one inline point is also 'in the region' of the line it is 'inline' with,
        // it has to be touching it
        a == 0 && self.p1.is_in_boundary(other)
            || b == 0 && self.p2.is_in_boundary(other)
            || c == 0 && other.p1.is_in_boundary(self)
            || d == 0 && other.p2.is_in_boundary(self)
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

    let print_each_check = false;

    let mut lines: Vec<Line> = Vec::new();
    for row in rows {
        lines.push(string_to_line(row))
    }

    let mut number_of_crosses = 0;

    let timer = SystemTime::now();
    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            let line1 = &lines[i];
            let line2 = &lines[j];
            let crosses = line1.crosses(line2);
            if crosses {
                number_of_crosses += 1;
            }
            if print_each_check {
                print!(
                    "Line1: (({:?},{:?})({:?},{:?}))\n Line2: (({:?},{:?})({:?},{:?}))\n crosses: {:?}\n\n",
                    line1.p1.x,
                    line1.p1.y,
                    line1.p2.x,
                    line1.p2.y,
                    line2.p1.x,
                    line2.p1.y,
                    line2.p2.x,
                    line2.p2.y,
                    crosses
                );
            }
        }
    }
    println!("Time elapsed: {:?}", timer.elapsed().unwrap().as_millis());
    println!("Number of crosses: {}", number_of_crosses);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 1.0, y: 0.0 }, Point { x: 1.0, y: 1.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));
    }

    #[test]
    fn case2() {
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: -1.0 }, Point { x: 1.0, y: 1.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));
    }

    #[test]
    fn case3() {
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: -1.0, y: -1.0 }, Point { x: 1.0, y: -1.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));
    }

    #[test]
    fn case4() {
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 1.0, y: 1.0 }, Point { x: 1.0, y: 1.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));
    }

    #[test]
    fn case5() {
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
    }

    #[test]
    fn case6() {
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: -1.0, y: 0.0 }, Point { x: 1.0, y: 0.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
    }

    #[test]
    fn case7_9() {
        // For all 4 coordinates of two lines, iterate over all points in a 7x7 lattice.
        // Then calculate the minimized ccw values and check if one of them is the impossible one.
        use p01_unique_cases::{Ccw, CcwCombination};
        let impossible1 = CcwCombination {
            ccw1: Ccw::Left,
            ccw2: Ccw::Middle,
            ccw3: Ccw::Middle,
            ccw4: Ccw::Middle,
        };
        let impossible2 = CcwCombination {
            ccw1: Ccw::Left,
            ccw2: Ccw::Right,
            ccw3: Ccw::Middle,
            ccw4: Ccw::Middle,
        };

        for a in (0..7).map(f64::from) {
            for b in (0..7).map(f64::from) {
                for c in (0..7).map(f64::from) {
                    for d in (0..8).map(f64::from) {
                        for e in (0..7).map(f64::from) {
                            for f in (0..7).map(f64::from) {
                                for g in (0..7).map(f64::from) {
                                    for h in (0..7).map(f64::from) {
                                        let l1 =
                                            Line::new(Point { x: a, y: b }, Point { x: c, y: d });
                                        let l2 =
                                            Line::new(Point { x: e, y: f }, Point { x: g, y: h });

                                        let ccw1 = p01_unique_cases::Ccw::from(ccw(
                                            &l1.p1, &l1.p2, &l2.p1,
                                        ));
                                        let ccw2 = p01_unique_cases::Ccw::from(ccw(
                                            &l1.p1, &l1.p2, &l2.p2,
                                        ));
                                        let ccw3 = p01_unique_cases::Ccw::from(ccw(
                                            &l2.p1, &l2.p2, &l1.p1,
                                        ));
                                        let ccw4 = p01_unique_cases::Ccw::from(ccw(
                                            &l2.p1, &l2.p2, &l1.p2,
                                        ));
                                        let minimized_ccw_combi =
                                            p01_unique_cases::CcwCombination {
                                                ccw1,
                                                ccw2,
                                                ccw3,
                                                ccw4,
                                            }
                                            .minimize();
                                        assert!(minimized_ccw_combi != impossible1);
                                        assert!(minimized_ccw_combi != impossible2);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn case8() {
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: -1.0, y: 0.0 }, Point { x: 1.0, y: 1.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
    }

    #[test]
    fn case10() {
        // 2 "Point-Lines"
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        let l2 = Line::new(Point { x: 0.0, y: -1.0 }, Point { x: 0.0, y: -1.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        let l2 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        let l2 = Line::new(Point { x: 0.0, y: 1.0 }, Point { x: 0.0, y: 1.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));

        // 1 Line, 1 "Point-Line"
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        let l2 = Line::new(Point { x: 0.0, y: -2.0 }, Point { x: 0.0, y: -1.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        let l2 = Line::new(Point { x: 0.0, y: -2.0 }, Point { x: 0.0, y: 0.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        let l2 = Line::new(Point { x: 0.0, y: -2.0 }, Point { x: 0.0, y: 2.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        let l2 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 2.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
        let l2 = Line::new(Point { x: 0.0, y: 1.0 }, Point { x: 0.0, y: 2.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));

        // 2 Lines
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: -1.0 }, Point { x: 0.0, y: -0.5 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: -0.5 }, Point { x: 0.0, y: 0.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: -0.25 }, Point { x: 0.0, y: 0.25 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.5 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: 0.25 }, Point { x: 0.0, y: 0.75 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: 0.5 }, Point { x: 0.0, y: 1.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: 0.75 }, Point { x: 0.0, y: 1.25 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: 1.0 }, Point { x: 0.0, y: 1.5 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: 1.5 }, Point { x: 0.0, y: 2.0 });
        assert!(!l1.crosses(&l2));
        assert!(!l2.crosses(&l1));

        let l1 = Line::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
        let l2 = Line::new(Point { x: 0.0, y: -2.0 }, Point { x: 0.0, y: 2.0 });
        assert!(l1.crosses(&l2));
        assert!(l2.crosses(&l1));
    }
}
