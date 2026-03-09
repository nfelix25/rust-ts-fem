interface Op {
    name?: string;
}

const _n: Op = {
    // Invalid with "exactOptionalPropertyTypes" enabled
    name: undefined,
};

// Type '{ name: undefined; }' is not assignable to type 'Op' with 'exactOptionalPropertyTypes: true'. Consider adding 'undefined' to the types of the target's properties.
//   Types of property 'name' are incompatible.
//     Type 'undefined' is not assignable to type 'string'.

// [name]?: [type] is an optional string, string or literally undefined/not ever defined/no value

const n: Op = {}; // Valid

// [name]: [type | undefined] is an required value that can have the type undefined
interface Pop {
    name: string | undefined;
}

const _p: Pop = {}; // Invalid
// Property 'name' is missing in type '{}' but required in type 'Pop'.

const p: Pop = {
    name: _p.name, // Valid via JS spec requirements (see below)
};

// Property Lookup: The engine checks if the specified key exists within the object's internal property list (and its prototype chain).
// If the property is not found, the engine is designed to return the primitive value undefined

function practice(num: number | undefined): number {
    return (num ?? 0) * 5;
}

function practice2(num: number | undefined): number | undefined {
    return num === undefined ? num : num * 5;
}

console.log(practice(undefined));
console.log(practice(10));

console.log(practice2(undefined));
console.log(practice2(10));

// Left func return sig as was when copy pasting and highlighted another TS case:
// Even though there is no possible way for undefined to be returned, the return type number | undefined is still treated as valid (could be number | undefined | string | Type etc etc...)
// Guessing that this is ultimately the same pattern that allows for valid wide types like any/unknown
function p_ind(nums: number[], i: number): number | undefined {
    return (nums[i] ?? i) * 5;
}

const nums = [1, 2, 3];

console.log(p_ind(nums, 2)); // nums[2] exists, 3 * 5 == 15
console.log(p_ind(nums, 10)); // nums[10] is undefined, 3 * 10 == 50
