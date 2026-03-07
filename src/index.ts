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
