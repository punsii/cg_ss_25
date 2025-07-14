use crate::lib::ccw::ccw;
use crate::lib::line::Line;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

#[derive(Clone, Debug, PartialEq, Copy)]
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

    pub fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        if len == 0.0 {
            Self { x: 0.0, y: 0.0 }
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, other: &Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
