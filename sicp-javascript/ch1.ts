

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

