enum Color {
    Red,
    Blue,
    Green,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Seeing red?"),
        Color::Blue => println!("Feeling blue?"),
        Color::Green => println!("Green with envy?"),
    }
}

fn main() {
    print_color(Color::Red);
    print_color(Color::Green);
    print_color(Color::Blue);
}
