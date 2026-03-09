#[derive(Debug)]
struct Item {
    count: i32,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    let mut item = Item { count: 0 };

    println!("{:?}", item);

    add_one(&mut item);

    println!("{:?}", item);

    let mut items = vec![Item { count: 0 }];

    // Expirement with order to see different borrow errors
    let first = items.get_mut(0);
    println!("{:?}", first);
    let second = items.get_mut(1);
    println!("{:?}", second);

    print_all(&items);

    // println!("{:?}", first); // This would cause an error due to a mix of immmutable and mutable ref existing to items[0]

    // Below wis borrow error as iter returns a reference to an iter, which references "self" (eg the vec![1, 2, 3]) --- The RHS is evaluated first, which means that entire line evaluates to a reference to vec which no longer exists
    //
    // let items = vec![1, 2, 3].iter().map(|x| x + 1);

    // That is why it is idiomatic and required to break out the assignment OR use collect (also a good example of why collect)
    let items = vec![1, 2, 3];
    let items = items.iter().map(|x| x + 1);

    println!("{:?}", items);
    println!("{:?}", items.collect::<Vec<_>>());

    let items: Vec<_> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
    println!("{:?}", items);
}
