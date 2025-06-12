use crate::lib::{ccw::ccw, line::Line};

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn is_in_boundary(&self, line: &Line) -> bool {
        let xmin = f64::min(line.p1.x, line.p2.x);
        let xmax = f64::max(line.p1.x, line.p2.x);
        let ymin = f64::min(line.p1.y, line.p2.y);
        let ymax = f64::max(line.p1.y, line.p2.y);

        self.x >= xmin && self.x <= xmax && self.y >= ymin && self.y <= ymax
    }

    pub fn is_in_polygon(&self, polygon: &Vec<Point>) -> bool {
        let point_not_in_polygon = Point { x: -1.0, y: -1.0 };
        let mut i = 1;
        while ccw(&point_not_in_polygon, self, &polygon[i]) == 0 {
            i += 1;
        }
        let mut s = 0;
        let mut lr = ccw(&point_not_in_polygon, self, &polygon[i]);
        for j in i + 1..polygon.len() {
            let lrnew = ccw(&point_not_in_polygon, self, &polygon[j]);
            if (lrnew - lr).abs() == 2 {
                lr = lrnew;
                if ccw(&polygon[j - 1], &polygon[j], &point_not_in_polygon)
                    * ccw(&polygon[j - 1], &polygon[j], self)
                    <= 0
                {
                    s += 1;
                }
            }
        }
        s % 2 != 0
    }
}
