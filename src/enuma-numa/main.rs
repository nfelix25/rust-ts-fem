#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Color {
    pub fn is_green(&self) -> bool {
        match self {
            Self::Green => true,
            _ => false,
        }
    }

    pub fn is_blue(&self) -> bool {
        if let Color::Blue = self {
            true
        } else {
            false
        }
    }

    pub fn is_yellow(&self) -> bool {
        if let Color::Yellow = self {
            true
        } else {
            false
        }
    }

    pub fn is_part_green(&self) -> bool {
        if Self::is_blue(self) || Self::is_yellow(self) {
            true
        } else {
            false
        }
    }
}

fn print_color(color: &Color) {
    match color {
        Color::Red => println!("Seeing red?"),
        Color::Blue => println!("Feeling blue?"),
        Color::Green => println!("Green with envy?"),
        Color::Yellow => println!("Ya yellow?"),
        _ => unreachable!("Just reminding myslef that unreachable exists"),
    }
}

fn is_string(str: &str) {
    match str {
        "secret" => println!("Wow, that's the secret!"),
        _ => println!("Bozo."),
    }
}

fn main() {
    let mut color = &Color::Red;
    print_color(color);

    color = &Color::Green;
    print_color(color);

    color = &Color::Blue;
    print_color(color);

    color = &Color::Yellow;
    print_color(color);

    println!("Color {:?} is green: {}", color, color.is_green());

    color = &Color::Green;
    println!("Color {:?} is green: {}", color, color.is_green());

    println!(
        "Color {:?} is part of green: {}",
        color,
        color.is_part_green()
    );

    color = &Color::Yellow;
    println!(
        "Color {:?} is part of green: {}",
        color,
        color.is_part_green()
    );

    is_string("password");
    is_string("secret");

    #[derive(Debug)]
    struct Custom {
        age: u8,
        name: String,
    }

    // Discriminate (the tag determining which variant it is) will be typed as the smallest uint required to cover all cases (2-256, u8 --- 257-65536, u16 --- etc... BUT the compiler will optimize this away as much as possible (eg hide the min bits in the padding in the struct) - hence the size 32 even though that is the size of the int ans String alone, techincally. To see this explicitly, use #[repr(c)] to force C memory which will output 40
    #[derive(Debug)]
    enum Item {
        Number(usize),
        String(String),
        MyCustom(Custom),
    }

    fn append(items: &mut Vec<Item>) {
        items.push(Item::String("Hello, FEM!".into()));
    }

    let mut items: Vec<Item> = vec![Item::Number(2)];

    append(&mut items);

    println!("Items: {:?}", items);

    // Print address of item 0 and item 1
    println!("Address of item 0: {:p}", &items[0]); // Eg as dec.. 105553175806180
    println!("Address of item 1: {:p}", &items[1]); // Eg as dec.. 105553175806112

    // Print size of item 0 and item 1
    println!("Size of item 0: {}", std::mem::size_of_val(&items[0]));
    println!("Size of item 1: {}", std::mem::size_of_val(&items[1]));

    // Print size of Item enum
    println!("Size of Item enum: {}", std::mem::size_of::<Item>());

    // Print sizes of each enum variant
    println!(
        "Size of Item.Number (usize): {}",
        std::mem::size_of::<usize>()
    );
    println!(
        "Size of Item.String (String): {}",
        std::mem::size_of::<String>()
    );
    println!(
        "Size of Item.MyCustom (Custom): {}",
        std::mem::size_of::<Custom>()
    );

    // Print difference in addresses
    println!(
        "Difference in addresses: {}",
        (&items[1] as *const Item as usize) - (&items[0] as *const Item as usize)
    );

    // behind the scenes an enum is just a C type union that will occupy the size of the largest variant, and a tag to determine which variant it is. In this case, the largest variant is the MyCustom variant, which will occupy the size of a Custom struct, and the tag will be an integer that determines which variant it is. The enum will be represented in memory as a struct that contains the tag and the data for the largest variant.

    let me = Custom {
        name: "Noel".into(),
        age: 42,
    };

    let me2 = Custom {
        name: "Bob".into(),
        age: 99,
    };

    items.push(Item::MyCustom(me));

    fn print_item(i: &Item) {
        match &i {
            Item::MyCustom(c) =>
                println!("I'm Custom! My name is {} and my age is {}", c.name, c.age),

            _ => println!("I am a String or Number, or something else you added and now won't be caught because you used a catch-all! My value is {:?}", i)
        }
    }

    fn print_item2(i: &Item) {
        match &i {
            Item::MyCustom(Custom { age, .. }) if *age == 42 => {
                println!("I'm exactly 42, wow! What an age {} is!", age)
            }
            Item::MyCustom(Custom { age, .. }) => println!("I'm Custom! And my age is {}", age),

            _ => (),
        }
    }

    fn print_item3(i: &Item) {
        match &i {
            Item::MyCustom(c) if c.name == "Bob" => {
                println!("I'm Bob... That's wierd, wow! My age is {}", c.age)
            }
            Item::MyCustom(Custom { name, .. }) => println!("I'm Custom! And my name is {}", name),

            _ => (),
        }
    }

    for i in items {
        print_item(&i);
        print_item2(&i);
        print_item3(&i);
    }
    {
        let bob = Item::MyCustom(me2);
        print_item2(&bob);
        print_item3(&bob);
    }
}
