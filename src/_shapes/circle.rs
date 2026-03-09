use super::point::Point;
use crate::_area::Area;

pub struct Circle {
    pub origin: Point,
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}
