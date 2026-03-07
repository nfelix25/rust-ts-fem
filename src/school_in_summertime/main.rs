// Emulate TS classes using struct and impl --- Key takeaway is the separation of data (struct) and behavior (impl)

mod person {
    use std::collections::HashSet;
    use std::sync::{LazyLock, Mutex};

    // LazyLock initializes this global value only on first access (once per program).
    // Mutex makes shared mutable access safe: lock() gives a temporary guard, and
    // the guard unlocks automatically when it goes out of scope.

    static USED_NAMES: LazyLock<Mutex<HashSet<String>>> =
        LazyLock::new(|| Mutex::new(HashSet::new()));

    #[derive(Debug)]
    pub enum PersonError {
        NameTaken(String),
        FriendIsSelf,
        RegistryPoisoned,
    }

    #[derive(Debug)]
    pub struct Person {
        // Equivalent to class properties
        pub name: String,
        pub age: i32,

        // Equivalent to private class properties
        friends: Vec<String>,
        nick_name: String,
    }

    impl Person {
        // Equivalent to static properties
        pub const SPECIES: &str = "Human";

        // Equivalent to private static properties
        const ACTUAL_SPECIES: &str = "Robot";

        // Equivalent to constructor
        pub fn new(name: String, age: i32) -> Result<Self, PersonError> {
            // Check if the name is already taken
            let mut registry = USED_NAMES
                .lock()
                .map_err(|_| PersonError::RegistryPoisoned)?;

            if registry.contains(&name) {
                return Err(PersonError::NameTaken(name));
            }
            registry.insert(name.clone());

            // Must let format borrow name before we can move it into the struct, so we create a new variable for the nickname
            let nick_name = format!("{} {}", Self::ACTUAL_SPECIES, name);

            Ok(Person {
                name,
                age,
                nick_name,
                friends: Vec::new(),
            })
        }

        // Equivaent to public static method
        pub fn make_friends(a: &mut Person, b: &mut Person) -> Result<(), PersonError> {
            if a.name == b.name {
                return Err(PersonError::FriendIsSelf);
            }

            a.friends.push(b.name.clone());

            b.friends.push(a.name.clone());

            Ok(())
        }

        // Equivalent-ish to private class method
        fn get_species() -> &'static str {
            Self::SPECIES
        }

        // Equivalent to public class methods
        pub fn birthday(&mut self) {
            self.age += 1;
        }

        pub fn greet(&self, whom: &Person) {
            let is_friend = self.friends.contains(&whom.name);
            if is_friend {
                println!(
                    "Hi {}, my friend! It's me, {}, the {}. I'm {} years old.",
                    whom.nick_name,
                    self.nick_name,
                    Self::ACTUAL_SPECIES,
                    self.age
                );
            } else {
                println!(
                    "Hi {}, nice to meet you! I'm {}, the {}. I'm {} years old.",
                    whom.name,
                    self.name,
                    Self::get_species(),
                    self.age
                );
            }
        }

        // Unique to Rust. This is a method that consumes the instance, meaning it takes ownership of self. This is useful for things like destructors or when you want to move the instance into another context.
        pub fn die(self) {
            // Handle manual "destructor" work like removing the name from the registry
            let mut registry = USED_NAMES
                .lock()
                .map_err(|_| PersonError::RegistryPoisoned)
                .unwrap();

            registry.remove(&self.name);

            println!("{} has died. Goodbye, cruel world!", self.name);
        }
    }
}

fn main() {
    use person::Person;

    // Must be mutable to call make_friends and birthday, which modify the instance. In Rust, mutability is explicit, so we have to declare these variables as mutable if we want to change them.
    let mut alice = Person::new("Alice".to_string(), 30).expect("Failed to create Alice");
    let mut bob = Person::new("Bob".to_string(), 25).expect("Failed to create Bob");

    alice.greet(&bob); // Not friends yet

    Person::make_friends(&mut alice, &mut bob).expect("Failed to make friends");

    alice.birthday(); // Alice has a birthday, now she's 31

    alice.greet(&bob); // Now they are friends

    // Attempting to create another person with the same name should fail
    match Person::new("Alice".to_string(), 22) {
        Ok(_) => println!("Unexpectedly created a second Alice!"),
        Err(person::PersonError::NameTaken(name)) => {
            println!("Name already taken: {}", name);
        }
        Err(e) => println!("Other error: {:?}", e),
    }

    alice.die(); // Alice dies, removing her name from the registry

    // Attempting to create another person with the same name should now pass
    match Person::new("Alice".to_string(), 22) {
        Ok(_) => println!("Expectedly created a new Alice!"),
        Err(person::PersonError::NameTaken(name)) => {
            println!("Name already taken: {}", name);
        }
        Err(e) => println!("Other error: {:?}", e),
    }
}
