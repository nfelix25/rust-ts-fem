// unreachable and todo

fn only_evens(x: i32) {
    if x % 2 == 1 {
        // thread 'main' (8333548) panicked at src/main.rs:6:9:
        // internal error: entered unreachable code: This will panic if reached
        unreachable!("This will panic if reached")
    }

    // thread 'main' (8331629) panicked at src/main.rs:9:5:
    // not yet implemented: This will also panic if reached, good for marking cases remaining to code
    todo!("This will also panic if reached, good for marking cases remaining to code")
}

fn main() {
    only_evens(1);
    only_evens(2); // Will only run if above line is commented
}
