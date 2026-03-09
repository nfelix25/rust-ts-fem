// Running .ts file directly with modern Node utilizes automatic type-stripping
// This results in:
// SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]: TypeScript enum is not supported in strip-only mode
//
// ```node --experimental-transform-types myfile.ts``` will work
// ts* will still work fine
// but ```const {enum_thing} as const``` is king

enum Color {
    Red,
    Blue,
    Green,
}

function printColor(color: Color) {
    switch (color) {
        case Color.Red: {
            console.log("Seeing red?");
            break;
        }
        case Color.Blue: {
            console.log("Feeling blue?");
            break;
        }
        case Color.Green: {
            console.log("Green with envy?");
            break;
        }
    }
}

printColor(Color.Red);
printColor(Color.Blue);
printColor(Color.Green);

type Custom = {
    age: number;
    name: string;
};

type Item = number | string | Custom;

const items: Item[] = [];

function append(items: Item[]) {
    items.push("Hello FEM!");
}

console.log(items);
append(items);
console.log(items);

const nums: number[] = [];
console.log(nums);
append(nums); // Does not throw a type error
console.log(nums);

// For arrays specifically, TypeScript is effectively permissive/covariant here:

// - if `number` is assignable to `Item`
// - then `number[]` is often assignable to `Item[]
//
// this is a classic “mutable covariance” problem. Rust rejects this kind of thing much more aggressively; TypeScript often accepts it for convenience.
//
// So even in strict mode, some unsafe assignments are still allowed, especially around:

// - mutable arrays
// - method parameter checking
// - structural typing compatibility

function append_safe<T extends Item>(items: T[], value: T) {
    items.push(value);
}

append_safe(nums, "Hello, FEM!"); // Argument of type 'string' is not assignable to parameter of type 'number'.

const foo = [() => {}];
append_safe(foo, () => {}); // Argument of type '(() => void)[]' is not assignable to parameter of type 'Item[]'.
