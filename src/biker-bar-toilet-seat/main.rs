use std::io::BufRead;

// https://doc.rust-lang.org/std/io/trait.BufRead.html
//
// pub trait BufRead: Read {
//     // Required methods
//     fn fill_buf(&mut self) -> Result<&[u8]>;
//     fn consume(&mut self, amount: usize);

//     // Provided methods
//     fn has_data_left(&mut self) -> Result<bool> { ... }
//     fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> Result<usize> { ... }
//     fn skip_until(&mut self, byte: u8) -> Result<usize> { ... }
//     fn read_line(&mut self, buf: &mut String) -> Result<usize> { ... }
//     fn split(self, byte: u8) -> Split<Self> ⓘ
//        where Self: Sized { ... }
//     fn lines(self) -> Lines<Self> ⓘ
//        where Self: Sized { ... }
// }

fn read_file_proper_ish() -> std::io::Result<()> {
    let path = match std::env::current_dir() {
        Ok(path) => path.join("src/lines"),
        Err(e) => {
            eprintln!("Bloop blorp error: {}", e);
            return Err(e);
        }
    };

    // Open the file in read-only mode
    let handle = std::fs::File::open(path).expect("Failed to open file");

    // Wrap the file in a Buffer
    let reader = std::io::BufReader::new(handle);

    // BufRead is a trait that BufReader implements;
    // allowing the below equivalent of "for line_result in std::io::BufRead::lines(reader)"
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => println!("{line}"),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(())
}

fn main() {
    read_file_proper_ish().expect("Failed to read file");

    println!("---");

    // Quick and straightforward, but reads the whole file into memory and no real error handling
    let file = std::fs::read_to_string("src/lines").expect("Failed to read file");
    file.lines().for_each(|ln| println!("{ln}"));

    // comparative TS
    // fs.readFileSync(path)
    //   .toString()
    //   .split("\n")
    //   .forEach((line) => console.log(line));

    println!("---");

    // Track and then print how long it takes to open file and store every other line 1000000 times

    let start = std::time::Instant::now();

    let mut temp: &str = "";

    // Will take on the order of ~15+ seconds
    for _ in 0..100_0000 {
        let file = std::fs::read_to_string("src/lines").expect("Failed to read file");
        file.lines()
            .enumerate()
            .filter(|(idx, _)| idx % 2 == 0)
            .for_each(|(_, _)| temp = "LINE");
    }
    // Every other line
    let file = std::fs::read_to_string("src/lines").expect("Failed to read file");
    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, ln)| println!("{ln}"));

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
