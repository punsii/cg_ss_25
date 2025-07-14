use crate::lib::common::EPSILON;
use crate::lib::line::Line;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct LineWithOrd {
    pub line: Line,
    pub sweep_x: f64,
}

impl LineWithOrd {
    pub fn new(line: Line, sweep_x: f64) -> Self {
        Self { line, sweep_x }
    }

    pub fn y_at(&self) -> f64 {
        let (x0, y0) = (self.line.p1.x, self.line.p1.y);
        let (x1, y1) = (self.line.p2.x, self.line.p2.y);
        if (x1 - x0).abs() < EPSILON {
            return y0;
        }
        y0 + ((self.sweep_x - x0) / (x1 - x0)) * (y1 - y0)
    }
}

impl Eq for LineWithOrd {}

impl PartialEq for LineWithOrd {
    fn eq(&self, other: &Self) -> bool {
        self.y_at().partial_cmp(&other.y_at()).unwrap() == Ordering::Equal
    }
}

impl PartialOrd for LineWithOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.y_at().partial_cmp(&other.y_at())
    }
}

impl Ord for LineWithOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
