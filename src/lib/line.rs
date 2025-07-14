use crate::lib::{ccw::ccw, point::Point};

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
    pub n: Option<Point>,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Line {
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

    pub fn crosses(&self, other: &Line) -> bool {
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
