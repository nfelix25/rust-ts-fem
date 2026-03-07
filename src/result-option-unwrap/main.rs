// Option, Result, and unwrap

fn some_value() {
    let mut x: Option<i32> = Some(5);

    let mut y: i32 = x.unwrap(); // This will panic if x is None, but allows typing to be inferred as i32

    println!("The value of x is: {:?}. The value unwrapped is {}", x, y);

    x = None;

    y = x.unwrap_or(10); // This will return 10 if x is None

    println!(
        "The value of x is: {:?}. The value unwrapped with default is {}",
        x, y
    );
}

fn result_ex() -> Result<u128, String> {
    let now = std::time::SystemTime::now();
    let ms = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time done fucked up") // Think of as a .catch block that is also a passthrough
        .as_millis();

    if ms % 2 == 1 {
        Ok(ms)
    } else {
        Err("CRAPPED OUT!".to_string())
    }
}

// The value of x is: Some(5). The value unwrapped is 5
// The value of x is: None. The value unwrapped with default is 10

// thread 'main' (8376101) panicked at src/main.rs:36:26:
// called `Result::unwrap()` on an `Err` value: "CRAPPED OUT!"
//
// OR
//
// The current time in milliseconds since the UNIX epoch is an odd number and: 1772850115005
//
// After adding the add'l cases a couple possible outputs are like:
//
// The current time in milliseconds since the UNIX epoch is an odd number and: 1772850290941
// Error: CRAPPED OUT!
// The current time in milliseconds since the UNIX epoch is an odd number or 0 and: 1772850292947
//

// The current time in milliseconds since the UNIX epoch is an odd number and: 1772850330949
// The current time in milliseconds since the UNIX epoch is an odd number and: 1772850331951
// Error: CRAPPED OUT!
// The current time in milliseconds since the UNIX epoch is an odd number or 0 and: 0

fn main() {
    some_value();
    let ms = result_ex().unwrap(); // This will panic if result_ex returns an Err, but allows typing to be inferred as u128

    println!(
        "The current time in milliseconds since the UNIX epoch is an odd number and: {}",
        ms
    );

    // Sleep for a second to increase the chances of getting an odd number for the next call

    std::thread::sleep(std::time::Duration::from_secs(1));

    // A better way to handle it would be to use a match statement to handle both the Ok and Err cases without panicking
    match result_ex() {
        Ok(ms) => println!(
            "The current time in milliseconds since the UNIX epoch is an odd number and: {}",
            ms
        ),
        Err(e) => println!("Error: {}", e),
    }

    // Sleep for a second to increase the chances of getting an odd number for the next call

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Or we could use unwrap_or_else to provide a default value or handle the error in a closure
    let ms = result_ex().unwrap_or_else(|e| {
        println!("Error: {}", e);
        0 // Return a default value of 0 if there was an error
    });

    println!(
        "The current time in milliseconds since the UNIX epoch is an odd number or 0 and: {}",
        ms
    );
}
