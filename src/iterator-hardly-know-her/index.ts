const foo = [1, 2, 3].map((x) => x + 1);

// Make a custom iterator
function* myterator(arr: number[]) {
  for (const x of arr) {
    yield x * x;
  }
}

const squares = myterator([1, 2, 3]);

for (const square of squares) {
  console.log(square);
}

console.log(foo);
