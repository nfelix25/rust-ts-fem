// This is very very bad practice as I would utilize threads instead, but this is copying what we did (roughly) in the Node version of this project. The idea is to spawn two child processes, one with a valid file and one with an invalid file, and see how they handle the errors.

fn create_child_process() {
    // Fail case with no args
    std::process::Command::new("cargo")
        .arg("run")
        .arg("is_child")
        .spawn()
        .expect("Failed to spawn child process");

    // Fail case with bad file
    std::process::Command::new("cargo")
        .arg("run")
        .arg("is_child")
        .arg("no_file")
        .spawn()
        .expect("Failed to spawn child process");

    std::process::Command::new("cargo")
        .arg("run")
        .arg("is_child")
        .arg("src/valid_file")
        .spawn()
        .expect("Failed to spawn child process");
}

fn main() {
    let filename = std::env::args().nth(2);
    let is_child = std::env::args().nth(1) == Some("is_child".to_string());

    // If not child process, create child process
    if !(is_child) {
        create_child_process();
        // Manually kill parent process after 2 seconds --- Otherwise default without thread.join is parent process hanging indefinitely
        std::thread::sleep(std::time::Duration::from_secs(2));
        std::process::exit(0);
    } else {
        println!("Child process received args: {:?}", std::env::args());
        match filename {
            Some(filename) => {
                println!("Child process received filename: {}", filename);
                if let Ok(contents) = std::fs::read_to_string(filename) {
                    // ::<>() is called "turbofish" and the :: is needed to allow the compiler to differentiate between specifying the type of the generic AND somethin glike the below:
                    // method<T>() --- < and > re both overloadable operators and () is the unit type
                    // so technically the above could be compiled as "is (method less than t) greater than unit?"
                    contents.lines().for_each(|ln| match ln.parse::<i32>() {
                        Ok(num) => println!("Printing number {num}"),
                        // Contrast with the ts .split() version, this will NOT print for the trailing newline
                        Err(e) => eprintln!("Failed to print \"{ln}\" with error: {e}"),
                    });
                    std::process::exit(0);
                } else {
                    eprintln!("Failed to read file");
                    std::process::exit(1);
                }
            }
            None => {
                eprintln!("Usage: cargo run <filename>");
                std::process::exit(1);
            }
        }
    }
}
