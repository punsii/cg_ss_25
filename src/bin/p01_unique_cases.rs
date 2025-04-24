#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Ccw {
    Left = -1,
    Middle = 0,
    Right = 1,
}
impl Ccw {
    const VALUES: [Self; 3] = [Self::Left, Self::Middle, Self::Right];
}
impl From<i32> for Ccw {
    fn from(i: i32) -> Self {
        if i < 0 {
            Ccw::Left
        } else if i == 0 {
            Ccw::Middle
        } else {
            Ccw::Right
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CcwCombination {
    pub ccw1: Ccw,
    pub ccw2: Ccw,
    pub ccw3: Ccw,
    pub ccw4: Ccw,
}
impl CcwCombination {
    pub fn minimize(&self) -> CcwCombination {
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
        if ccw1 == Ccw::Right {
            ccw1 = Ccw::Left;
            ccw2 = Ccw::Left;
        }
        if ccw3 == Ccw::Right {
            ccw3 = Ccw::Left;
            ccw4 = Ccw::Left;
        }
        if ccw1 == Ccw::Middle && ccw2 == Ccw::Right {
            ccw1 = Ccw::Left;
            ccw2 = Ccw::Middle;
        }
        if ccw3 == Ccw::Middle && ccw4 == Ccw::Right {
            ccw3 = Ccw::Left;
            ccw4 = Ccw::Middle;
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
