mod _area;
mod _shapes;

use _area::Area;
use _shapes::{circle::Circle, point::Point, rect::Rect};

fn main() {
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
    println!("{}", rect.to_string());

    println!("Default Rectangle: {}", Rect::default());

    // Print points with iter

    Rect::default()
        .into_iter()
        .enumerate()
        .for_each(|(idx, point)| println!("Point #{}: {}", idx + 1, point));

    for point in &rect {
        println!("Point: {point}");
    }

    // Able to print after iter now that iter also has case for reference and lifetimes -- Could also use with derived Copy
    println!("{rect}");
}
