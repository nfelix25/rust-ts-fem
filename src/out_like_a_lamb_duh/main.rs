fn main() {
    let arr = [1, 2, 3];

    // Map gets the item directly, while filter gets a reference to the item, so we need to dereference it to compare it to 2
    let fil: Vec<_> = arr.iter().map(|x| x * 2).filter(|x| *x > 2).collect();

    // [4, 6]
    println!("{:?}", fil)
}
