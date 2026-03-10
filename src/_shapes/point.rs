use crate::_collidable::Collidable;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct PointIter {
    points: Vec<Point>,
    idx: usize,
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        self.points.get(idx).copied()
    }
}

impl From<Vec<Point>> for PointIter {
    fn from(value: Vec<Point>) -> Self {
        PointIter {
            points: value,
            idx: 0,
        }
    }
}

pub trait Points {
    fn points(&self) -> PointIter;
}

pub trait Contains {
    fn contains_point(&self, other: Point) -> bool;
}

impl<T, V> Collidable<T> for V
where
    T: Points,   // Shape to check for collision with --- Need to have points
    V: Contains, // Shape to check if has been collided with -- Needs to see if it contains points from T
{
    fn collide(&self, other: &T) -> bool {
        for point in other.points() {
            if self.contains_point(point) {
                return true;
            }
        }
        false
    }
}
