

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