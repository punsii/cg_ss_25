#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum CCW {
    Left = -1,
    Middle = 0,
    Right = 1,
}
impl CCW {
    const VALUES: [Self; 3] = [Self::Left, Self::Middle, Self::Right];
}

#[derive(Debug, PartialEq)]
struct CcwCombination {
    ccw1: CCW,
    ccw2: CCW,
    ccw3: CCW,
    ccw4: CCW,
}
impl CcwCombination {
    fn minimize(&self) -> CcwCombination {
        let mut ccw1 = self.ccw1;
        let mut ccw2 = self.ccw2;
        let mut ccw3 = self.ccw3;
        let mut ccw4 = self.ccw4;

        if ccw1 > ccw2 {
            // reverse first line
            ccw2 = ccw1;
            ccw1 = self.ccw2;
        }
        if ccw3 > ccw4 {
            // reverse second line
            ccw4 = ccw3;
            ccw3 = self.ccw4;
        }

        // Mirror to the left
        if ccw1 == CCW::Right {
            ccw1 = CCW::Left;
            ccw2 = CCW::Left;
        }
        if ccw3 == CCW::Right {
            ccw3 = CCW::Left;
            ccw4 = CCW::Left;
        }
        if ccw1 == CCW::Middle && ccw2 == CCW::Right {
            ccw1 = CCW::Left;
            ccw2 = CCW::Middle;
        }
        if ccw3 == CCW::Middle && ccw4 == CCW::Right {
            ccw3 = CCW::Left;
            ccw4 = CCW::Middle;
        }

        if ccw1 > ccw3 || (ccw1 == ccw3 && ccw2 > ccw4) {
            // swap first and second line
            CcwCombination {
                ccw1: ccw3,
                ccw2: ccw4,
                ccw3: ccw1,
                ccw4: ccw2,
            }
        } else {
            CcwCombination {
                ccw1,
                ccw2,
                ccw3,
                ccw4,
            }
        }
    }
}

fn main() {
    let mut all_cases: Vec<CcwCombination> = vec![];
    for ccw1 in CCW::VALUES {
        for ccw2 in CCW::VALUES {
            for ccw3 in CCW::VALUES {
                for ccw4 in CCW::VALUES {
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
