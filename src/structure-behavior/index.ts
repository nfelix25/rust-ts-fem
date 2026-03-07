interface MyStringStructure {
  value: string;
  address: number;
}

interface MyStringBehavior {
  copy: () => MyStringClass;
}

interface MyString extends MyStringStructure, MyStringBehavior {}

interface MyStringStaticStructure {
  next_address: number;
}

interface MyStringStaticBehavior {
  new (value: string): MyString;
}

function staticImplements<T>() {
  console.log("Applying staticImplements decorator");
  return <U extends T>(constructor: U) => constructor;
}

@staticImplements<MyStringStaticBehavior & MyStringStaticStructure>()
class MyStringClass implements MyString {
  static next_address: number = 0;
  value: string;
  address: number;

  constructor(value: string) {
    this.value = value;
    this.address = MyStringClass.next_address++;
  }

  copy() {
    return new MyStringClass(this.value);
  }
}

const myStr = new MyStringClass("Hello, World!");
const myStrCopy = myStr.copy();

console.log(myStr.value); // Hello, World!
console.log(myStrCopy.value);

console.log(myStr.address); // 0
console.log(myStrCopy.address); // 1
