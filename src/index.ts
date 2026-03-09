import { inspect } from "util";

type AbstractConstructor<T = {}> = abstract new (...args: any[]) => T;

interface Printable {
    print(): string;
}

// Settling on the existing TS error to avoid completely seperate implementations for concrete and abstract
function Printable<TBase extends AbstractConstructor<Printable>>(Base: TBase) {
    return class extends Base {
        [inspect.custom]() {
            return this.print();
        }

        override toString() {
            return this.print();
        }

        [Symbol.toStringTag]() {
            return this.print();
        }
    };
}

class Point {
    constructor(
        public x: number,
        public y: number,
    ) {}

    print() {
        return `(${this.x}, ${this.y})`;
    }
}

class PrintablePoint extends Printable(Point) {}

interface Area {
    area(): number;
}

abstract class Shape implements Area {
    origin: Point;
    constructor(x: number, y: number) {
        this.origin = new PrintablePoint(x, y);
    }

    abstract area(): number;

    print() {
        return `This shape has origin ${this.origin} and an area of ${this.area()}`;
    }
}

abstract class PrintableShape extends Printable(Shape) {}

class Rectangle extends PrintableShape {
    constructor(
        public width: number,
        public height: number,

        x: number,
        y: number,
    ) {
        super(x, y);
    }

    area(): number {
        return this.width * this.height;
    }
}

class Circle extends PrintableShape {
    constructor(
        public radius: number,

        x: number,
        y: number,
    ) {
        super(x, y);
    }

    area(): number {
        return this.radius * this.radius * Math.PI;
    }
}

const rect = new Rectangle(1, 2, 3.5, 4.5);
const circ = new Circle(1, 2, 3.14);

console.log(rect);
console.log(circ);
