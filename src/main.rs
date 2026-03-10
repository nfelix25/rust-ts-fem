mod _area;
mod _collidable;
mod _shapes;

use _area::Area;
use _shapes::{
    circle::Circle, point::Contains, point::Point, point::PointIter, point::Points, rect::Rect,
};

use crate::_collidable::Collidable;
use std::str::FromStr;

use anyhow::Result;

fn main() -> Result<()> {
    let rect = Rect {
        origin: Point { x: 1.5, y: 0.5 },
        width: 7.0,
        height: 3.0,
    };

    let circ = Circle {
        origin: Point { x: 1.0, y: 2.0 },
        radius: std::f64::consts::PI,
    };

    println!("Area of rect: {}", rect.area());
    println!("Area of circ: {}", circ.area());
    println!("Area of PI: {}", std::f64::consts::PI.area());

    // Implementing Display gives us to_string as well via the ToString trait
    // Literally, "This trait is automatically implemented for any type which implements the Display trait. As such, ToString shouldn’t be implemented directly: Display should be implemented instead, and you get the ToString implementation for free."
    //
    // Clippy complains if to_string is called when display exists
    // println!("{}", rect.to_string());
    println!("{rect}");

    println!("Default Rectangle: {}", Rect::default());

    // Print points with iter

    Rect::default()
        .points()
        .enumerate()
        .for_each(|(idx, point)| println!("Point #{}: {}", idx + 1, point));

    for point in rect.points() {
        println!("Point: {point}");
    }

    // Able to print after iter now that iter also has case for reference and lifetimes -- Could also use with derived Copy
    println!("{rect}");

    let p1 = Point { x: 0.0, y: 0.0 };
    println!(
        "Rectangle contains point {p1}: is {}",
        rect.contains_point(p1)
    );

    let p2 = Point { x: 10.0, y: 10.0 };
    println!(
        "Rectangle contains point {p2}: is {}",
        rect.contains_point(p2)
    );

    println!(
        "Circle origin: {}, radius: {}, area: {}",
        circ.origin,
        circ.radius,
        circ.area()
    );

    let p3 = Point {
        x: circ.origin.x + circ.radius / 2.0,
        y: circ.origin.y + circ.radius / 2.0,
    };

    let p4 = Point {
        x: circ.origin.x + circ.radius * 2.0,
        y: circ.origin.y + circ.radius * 2.0,
    };

    println!("Circle contains point {p3}: is {}", circ.contains_point(p3));

    println!("Circle contains point {p4}: is {}", circ.contains_point(p4));

    // Check shape combinations for collisions
    println!("Circle circ collides with rect: is {}", circ.collide(&rect));
    println!("Rect rect collides with circle: is {}", rect.collide(&circ));

    let c1 = Circle {
        origin: Point { x: 0.0, y: 0.0 },
        radius: 1.0,
    };

    let r1 = Rect {
        origin: Point { x: 3.0, y: 3.0 },
        width: 1.0,
        height: 1.0,
    };

    println!("Circle c1 collides with rect: is {}", c1.collide(&r1));
    println!("Rect r1 collides with circle: is {}", r1.collide(&c1));

    #[derive(Clone, Copy, Debug)]
    enum Shape {
        Circle(Circle),
        Rect(Rect),
    }

    impl FromStr for Shape {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

            match shape {
                "rect" => Ok(Shape::Rect(data.parse()?)),
                "circle" => Ok(Shape::Circle(data.parse()?)),
                _ => Err(anyhow::anyhow!("Bad!!!!!!!!!!")),
            }
        }
    }

    impl Points for Shape {
        fn points(&self) -> PointIter {
            match self {
                Shape::Circle(c) => c.points(),
                Shape::Rect(r) => r.points(),
            }
        }
    }

    impl Contains for Shape {
        fn contains_point(&self, point: Point) -> bool {
            match self {
                Shape::Circle(c) => c.contains_point(point),
                Shape::Rect(r) => r.contains_point(point),
            }
        }
    }

    // Read in 4 shapes from file "shapes"
    let shapes: Vec<Shape> = std::fs::read_to_string("src/shapes")
        .expect("failed to parse")
        .lines()
        // .for_each(|line| println!("Line: {line}"));
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect();

    for i in 1..=2 {
        // Print out cillisions between shapes[i], shapes[i - 1], and shapes[i + 1]
        println!(
            "Shape {} collides with shape {}: is {}",
            i,
            i - 1,
            shapes[i].collide(&shapes[i - 1])
        );
        println!(
            "Shape {} collides with shape {}: is {}",
            i,
            i + 1,
            shapes[i].collide(&shapes[i + 1])
        );

        let adj = vec![shapes[i - 1], shapes[i + 1]];

        // Now check both at once using collides
        println!(
            "Shape {} collides with either is: {}",
            i,
            shapes[i].collides(&adj)
        );
    }

    Ok(())
}
