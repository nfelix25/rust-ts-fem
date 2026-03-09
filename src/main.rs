mod _area;
mod _shapes;

use _area::Area;
use _shapes::{circle::Circle, point::Point, rect::Rect};

fn main() {
    let rect = Rect {
        origin: Point { x: 1, y: 2 },
        width: 3.5,
        height: 4.5,
    };

    let circ = Circle {
        origin: Point { x: 1, y: 2 },
        radius: std::f64::consts::PI,
    };

    println!("Area of rect: {}", rect.area());
    println!("Area of circ: {}", circ.area());
    println!("Area of PI: {}", std::f64::consts::PI.area());
}
