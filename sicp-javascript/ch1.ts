

function exercise12() {
    return (5 + 4 + (2 - (3 - (6 + (4/5)))))
    /
    (3 * (6-2) * (2-7))
}

function exercise13(a, b, c) {
    const l = [a, b, c].sort();

    return l[1]*l[1] + l[2]*l[2];
}


console.log(exercise12());
console.log(exercise13(2, 4, 1));

// https://sourceacademy.org/sicpjs/1.1.7

// Exercise 1.6
// Didn't immediately get it.
// TODO read up on applicative order evaluation

function square(x) {
    return x*x;
}

function is_good_enough(guess, x) {
    return Math.abs(square(guess) - x) < 0.001;
}

function sqrt_iter(guess, x) {
    return is_good_enough(guess, x)
        ? guess
        : sqrt_iter(improve(guess, x), x);
}

function improve(guess, x) {
    return average(guess, x / guess);
}

function average(x, y) {
    return (x + y) / 2;
}

function sqrt(x) {
    return sqrt_iter(1, x);
}

console.log("9", sqrt(9));
console.log("100 + 37", sqrt(100 + 37));
console.log("sqrt2 + sqrt3", sqrt(sqrt(2) + sqrt(3)));
console.log("1000", square(sqrt(1000)));

// Exercise 1.7

console.log("");
console.log("Exercise 1.7");
console.log("0.0001", sqrt(0.0001));
console.log("15241578750190521", sqrt(15241578750190521));

function is_good_enough_alt(guess, x) {
    return Math.abs(square(guess) - x) < 0.001;
}

function sqrt_alt(x) {
    return sqrt_iter_alt(1, x);
}

function sqrt_iter_alt(guess, x) {
    let new_guess = improve(guess, x);

    if (Math.abs(guess - new_guess) < 0.0001) {
        return new_guess;
    } else {
        return sqrt_iter_alt(new_guess, x);
    }
}

console.log("0.0001 with alt", sqrt_alt(0.0001));
console.log("3 normal", sqrt(3));
console.log("3 with alt", sqrt_alt(3));

// Exercise 1.8

function cube_root_iter(y, x) {
    let y_accent = improve_cube(y, x);

    if (Math.abs(y - y_accent) < 0.001) {
        return y_accent;
    } else {
        return cube_root_iter(y_accent, x);
    }
}

function improve_cube(y, x) {
    return ((x / square(y)) + (2*y))
    /
    3;
}

function cube_root(x) {
    return cube_root_iter(1, x);
}

console.log("");
console.log("Exercise 1.8");

console.log("Cube root 27", cube_root(27));

// Exercise 1.10

function A(x, y) {
    return y === 0
    ? 0
    : x === 0
    ? 2 * y
    : y === 1
    ? 2
    : A(x-1, A(x, y-1));
}

console.log("A1,10", A(1, 10));
console.log("A2,4", A(2, 4));
console.log("A3,3", A(3, 3));

// Exercise 1.11

function f_rec(n) {
    if (n < 3) {
        return n;
    } else {
        return f_rec(n-1) + 2 * f_rec(n-2) + 3 * f_rec(n-3)
    }
}


function f_iter(n) {
    if (n < 3) {
        return n;
    } else {
        return f_iter_helper(0, 1, 2, n-2);
    }
}

function f_iter_helper(a, b, c, count) {
    return count === 0
    ? c
    : f_iter_helper(b, c, c + 2 * b + 3 * a, count-1);
}


console.log("");
console.log("Exercise 1.11");
console.log("f_rec 24", f_rec(24));

console.log("f_rec 4", f_rec(8));
console.log("f_iter 4", f_iter(8));

// Exercise 1.12

console.log("");
console.log("Exercise 1.12");

function pascal_triangle(previous_row: number[], n: number) {
    console.log(previous_row.join(' '));

    if (n == 0) {
        return;
    } else {
        let new_row: number[] = [];
        new_row.push(1);

        for (let i = 0; i < previous_row.length-1; i++) {
            new_row.push(previous_row[i] + previous_row[i+1]);
        }

        new_row.push(1);

        pascal_triangle(new_row, n-1);
    }
}

pascal_triangle([1], 10);

// Exercise 1.13

// Don't see the point of doing this

// Exercise 1.14

// Also noping out here


// Exercise 1.15

function cube(x) {
    return x * x * x;
}

function p(x) {
    console.log("Call with ", x);

    return 3*x - 4*cube(x);
}

function sine(angle) {

    return ! (Math.abs(angle) > 0.1)
            ? angle :
            p(sine(angle / 3));
}

console.log("");
console.log("Exercise 1.15");

console.log("sine(12.15)", sine(12.15));
// This seems to be wrong with the text but whatever.


// Exercise 1.19
// Skipping all of this. I'm not here to revisit my analysis classes.

// Exercise 1.29