use std::{fmt::Display, str::FromStr};

use super::point::{Contains, Point, PointIter, Points};
use crate::_area::Area;

#[derive(Clone, Copy, Debug)]
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

impl Default for Rect {
    fn default() -> Self {
        Rect {
            origin: Point { x: 0.0, y: 0.0 },
            width: 10.0,
            height: 10.0,
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle ({}, {}): {}x{} - {}",
            self.origin.x,
            self.origin.y,
            self.width,
            self.height,
            self.area()
        )
    }
}

impl Rect {
    pub fn get_point(&self, point: usize) -> Result<Point, String> {
        if point == 0 || point > 4 {
            return Err(format!("Rect only has 4 points, but got {}", point));
        }

        let mults: (f64, f64) = match point {
            1 => (1.0, 1.0),
            2 => (-1.0, 1.0),
            3 => (-1.0, -1.0),
            4 => (1.0, -1.0),
            _ => unreachable!(),
        };

        Ok(Point {
            x: self.origin.x + mults.0 * self.width / 2.0,
            y: self.origin.y + mults.1 * self.height / 2.0,
        })
    }

    fn points(self) -> PointIter {
        let mut vec: Vec<Point> = Vec::new();

        for i in 1..=4 {
            vec.push(self.get_point(i).unwrap())
        }

        vec.into()
    }
}

// Point iter is able to replace all RectIter ... relaed code

// pub struct RectIter {
//     points: PointIter,
//     idx: usize,
// }

// impl RectIter {
//     fn new<'a>(rect: &'a Rect) -> Self {
//         RectIter {
//             points: rect.points(),
//             idx: 0,
//         }
//     }
// }

// allows for ret.into() to infer the type of RectIter and call the from function to create it
// impl From<Rect> for RectIter {
//     fn from(rect: Rect) -> Self {
//         RectIter {
//             points: rect.points(),
//             idx: 0,
//         }
//     }
// }

// impl Iterator for RectIter {
//     type Item = Point;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx >= 4 {
//             return None;
//         }
//         let p = self.points.nth(self.idx).unwrap();
//         self.idx += 1;
//         Some(p)
//     }
// }

// impl IntoIterator for Rect {
//     type Item = Point;
//     type IntoIter = RectIter;

//     fn into_iter(self) -> Self::IntoIter {
//         self.into()
//     }
// }

// impl<'a> IntoIterator for &'a Rect {
//     type Item = Point;
//     type IntoIter = RectIter;

//     fn into_iter(self) -> Self::IntoIter {
//         RectIter::new(self)
//     }
// }

impl Contains for Rect {
    fn contains_point(&self, Point { x, y }: Point) -> bool {
        // Should just determine distances, but being lazy
        let mut points = self.points();
        let top_left = points.nth(1).unwrap();
        let bottom_right = points.nth(1).unwrap();

        x >= top_left.x && x <= bottom_right.x && y >= bottom_right.y && y <= top_left.y
    }
}

impl Points for Rect {
    fn points(&self) -> PointIter {
        (*self).points()
    }
}

impl FromStr for Rect {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!(
                "Rect should be in format 'x y width height', but got '{}'",
                s
            ));
        }

        Ok(Rect {
            origin: Point {
                x: parts[0].parse()?,
                y: parts[1].parse()?,
            },
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        })
    }
}
