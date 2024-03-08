// * [ ] https://youtu.be/zer1aFgj4aU?si=A845ZyBfcHgsUGKX
// * [ ] https://youtu.be/Bsdl_NKbNnU?si=v45DY04xKptnxpWU

// 2. The product of two object in a poset is the meet of the two objects.

// 3. The coproduct of two objects in a poset is the join of the two objects.

// 4. Implement the equivalent of Haskell Either as a generic type in your favorite language

#[derive(Debug)]
enum Either<A, B> {
    Left(A),
    Right(B),
}

// 5. Show that Either is a “better” coproduct than int equipped with two injections
// Answer: int m(Either const &e) {
//   e.is_left() ? e.left() : e.right() ? 0 : 1;
// };
// m effectively wraps the two injections and maintains the Either property.

// 6. Continuing the previous problem: How would you argue that int with the two injections i and j cannot be “better” than Either?
// Answer: There needs to be a m that can take the int from i' and j' and construct an Either object from it with either an int or a bool. But because the {0, 1} values are ambiguous (they could have come from either function), it is not possible to construct this m.

// 7. Still continuing: What about these injections?
// I think these are fine at least because the bool values are not overwritten. So it's possible to write an m that turns these results back into a clean Either.
// That would make them the same? Equal up to isomorphism?

fn main() {
    let a: Either<usize, usize> = Either::Left(1);
    let b: Either<usize, usize> = Either::Right(2);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}
