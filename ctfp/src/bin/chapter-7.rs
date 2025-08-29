// 3. Implement the reader functor in your second favorite language
#[derive(Debug)]
struct Reader<A, B> {
    e: A,
    a: B,
}

fn fmap<A, B, F, C>(f: F, r: Reader<A, B>) -> Reader<A, C>
where
    F: Fn(B) -> C,
{
    Reader { e: r.e, a: f(r.a) }
}

fn main() {
    let double_string = |x: usize| -> String { (x * 2).to_string() };

    let input_reader = Reader { e: "Test", a: 2 };

    let output_reader = fmap(double_string, input_reader);

    println!("output_reader = {:?}", output_reader)
}

// Prove the functor laws for the list functor.
// TODO
