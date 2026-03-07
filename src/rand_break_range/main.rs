use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Hello, world!");

    let mut i = 0;
    let range = 0..=10; // Range is a first class citizen in Rust, and can be used in many places where you might use a number or a collection.

    loop {
        println!("This loop will run forever! #{i}");
        if i % 10 == rng.gen_range(range.clone()) {
            println!("Breaking the loop!");
            break;
        }
        i += 1;
    }

    println!("Broke")
}
