use std::str::FromStr;

use super::point::{Contains, Point, PointIter, Points};
use crate::_area::Area;

#[derive(Clone, Copy, Debug)]
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
        vec![self.origin].into()
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!(
                "Circle should be in format 'x y radius', but got '{}'",
                s
            ));
        }

        Ok(Circle {
            origin: Point {
                x: parts[0].parse()?,
                y: parts[1].parse()?,
            },
            radius: parts[2].parse()?,
        })
    }
}
