use crate::lib::{common::EPSILON, point::Point};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Ccw {
    Left = -1,
    Middle = 0,
    Right = 1,
}
impl Ccw {
    pub const VALUES: [Self; 3] = [Self::Left, Self::Middle, Self::Right];
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

pub fn ccw_value(p: &Point, q: &Point, r: &Point) -> f64 {
    p.x * q.y - p.y * q.x + q.x * r.y - q.y * r.x + p.y * r.x - p.x * r.y
}

pub fn ccw(p: &Point, q: &Point, r: &Point) -> i32 {
    let ccw = ccw_value(p, q, r);

    if ccw < -EPSILON {
        -1
    } else if ccw < EPSILON {
        0
    } else {
        1
    }
}

pub fn crossing_point(a1: &Point, a2: &Point, b1: &Point, b2: &Point) -> Point {
    if a1 == b1 && a2 == b2 {
        return a1.clone();
    }
    if a1.x == a2.x && a1.y == a2.y {
        return a1.clone();
    } else if b1.x == b2.x && b1.y == b2.y {
        return b1.clone();
    }
    let d1 = ccw_value(b1, a1, b2).abs();
    let d2 = ccw_value(b2, a2, b1).abs();
    let b1_b2 = ((b2.x - b1.x) * (b2.x - b1.x) + (b2.y - b1.y) * (b2.y - b1.y)).sqrt();
    let h1 = d1 / b1_b2;
    let h2 = d2 / b1_b2;
    let factor = h1 / (h1 + h2);
    Point {
        x: a1.x + (a2.x - a1.x) * factor,
        y: a1.y + (a2.y - a1.y) * factor,
    }
}
