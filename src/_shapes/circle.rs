use super::point::{Contains, Point, PointIter, Points};
use crate::_area::Area;

pub struct Circle {
    pub origin: Point,
    pub radius: f64,
}

impl Contains for Circle {
    fn contains_point(&self, Point { x, y }: Point) -> bool {
        let dx = self.origin.x - x;
        let dy = self.origin.y - y;
        let dist = (dx * dx + dy * dy).sqrt();

        dist <= self.radius
    }
}

impl Points for Circle {
    fn points(&self) -> PointIter {
        return vec![self.origin].into();
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}
