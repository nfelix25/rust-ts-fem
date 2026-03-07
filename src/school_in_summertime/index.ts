class Person {
  static #actualSpecies: string = "Robot";
  static species: string = "Human";

  #nickName: string;

  name: string;
  age: number;
  friends: Person[] = [];

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
    this.#nickName = `${Person.#actualSpecies} ${name}`;
  }

  birthday() {
    this.age++;
  }

  static getSpecies(otherSpecies: string) {
    return `${otherSpecies === Person.#actualSpecies ? Person.#actualSpecies : Person.species}`;
  }

  greet(addressing: Person) {
    console.log(
      `Hello, my name is ${this.friends.includes(addressing) ? this.#nickName : this.name} and I am ${this.age} years old.`,
    );
  }

  makeFriend(friend: Person) {
    this.friends.push(friend);
    friend.friends.push(this);
  }
}

const alice = new Person("Alice", 30);
const bob = new Person("Bob", 25);
const charlie = new Person("Charlie", 35);

alice.makeFriend(bob);

alice.greet(alice); // Hello, my name is Robot Alice and I am 30 years old.
alice.greet(bob); // Hello, my name is Robot Alice and I am 30 years old.
alice.greet(charlie); // Hello, my name is Alice and I am 30 years old.

console.log(Person.getSpecies("Robot")); // Robot
console.log(Person.getSpecies("Alien")); // Human
