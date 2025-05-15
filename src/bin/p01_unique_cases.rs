use cg_ss_25::lib::ccw::{Ccw, CcwCombination};

fn main() {
    let mut all_cases: Vec<CcwCombination> = vec![];
    for ccw1 in Ccw::VALUES {
        for ccw2 in Ccw::VALUES {
            for ccw3 in Ccw::VALUES {
                for ccw4 in Ccw::VALUES {
                    all_cases.push(CcwCombination {
                        ccw1,
                        ccw2,
                        ccw3,
                        ccw4,
                    })
                }
            }
        }
    }
    let mut result: Vec<CcwCombination> = vec![];
    for case in all_cases {
        if !result.contains(&case.minimize()) {
            result.push(case.minimize())
        }
    }

    for case in result {
        println!("{:?}", case);
    }
}
