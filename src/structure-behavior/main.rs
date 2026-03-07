struct MyString {
    value: String,
}

trait Copyable {
    fn copy(&self) -> Self;
}

impl Copyable for MyString {
    fn copy(&self) -> Self {
        MyString {
            value: self.value.clone(),
        }
    }
}

fn main() {
    let original = MyString {
        value: "Hello".to_string(),
    };
    let s: &[u8] = &original.value;
    let copy = original.copy();

    println!("Address of original: {:p}", &original);
    println!("Address of copy: {:p}", &copy);
}
