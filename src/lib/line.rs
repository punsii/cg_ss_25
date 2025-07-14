use crate::lib::{ccw::ccw, point::Point};

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
    pub n: Point,
    pub a: f64,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Line {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;

        let mut n = Point { x: -dy, y: dx }.normalize();
        let mut a = n.x * p1.x + n.y * p1.y;

        // Flip normal if pointing toward origin
        if a < 0.0 {
            n.x = -n.x;
            n.y = -n.y;
            a = -a;
        }

        Line { p1, p2, n, a }
    }

    pub fn point_distance(&self, point: Point) -> f64 {
        self.n.x * point.x + self.n.y * point.y - self.a
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

    pub fn length(&self) -> f64 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        (dx * dx + dy * dy).sqrt()
    }
}
