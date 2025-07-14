use std::time::{Instant, SystemTime};

use cg_ss_25::lib::data::read_lines_from_file;

fn main() {
    let print_each_check = false;

    let mut files = Vec::new();
    files.push("../data/01/s_1000_1.dat");
    files.push("../data/01/s_10000_1.dat");
    files.push("../data/01/s_100000_1.dat");
    files.push("../data/01/s_1000_10.dat");

    for file in files {
        // let lines = read_lines_from_file("../data/01/s_1000_10.dat");
        println!("{}", file.split("/").last().unwrap().split(".").nth(0).unwrap());
        let lines = read_lines_from_file(file);

        let mut number_of_crosses = 0;

        let timer = Instant::now();
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
        println!("Time elapsed: {:?}", timer.elapsed());
        println!("Number of crosses: {}", number_of_crosses);
    }
}

#[cfg(test)]
mod tests {
    use cg_ss_25::lib::{
        ccw::{Ccw, CcwCombination, ccw},
        line::Line,
        point::Point,
    };

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

                                        let ccw1 = Ccw::from(ccw(&l1.p1, &l1.p2, &l2.p1));
                                        let ccw2 = Ccw::from(ccw(&l1.p1, &l1.p2, &l2.p2));
                                        let ccw3 = Ccw::from(ccw(&l2.p1, &l2.p2, &l1.p1));
                                        let ccw4 = Ccw::from(ccw(&l2.p1, &l2.p2, &l1.p2));
                                        let minimized_ccw_combi = CcwCombination {
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
