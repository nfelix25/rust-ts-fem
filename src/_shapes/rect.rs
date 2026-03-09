use super::point::Point;
use crate::_area::Area;

pub struct Rect {
    pub origin: Point,
    pub width: f64,
    pub height: f64,
}

impl Area for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
