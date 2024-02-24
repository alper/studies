// fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
// where
//     F: Fn(A) -> B,
//     G: Fn(B) -> C,
// {
//     move |x| g(f(x))
// }

use std::hash::Hash;
use std::{collections::HashMap, rc::Rc};

fn memoize<A, B>(mut f: impl FnMut(A) -> B) -> impl FnMut(A) -> B
where
    A: Eq + Hash + Clone,
    B: Clone,
{
    let mut map = HashMap::new();

    move |a| map.entry(a).or_insert_with_key(|a| f(a.clone())).clone()
}

fn main() {
    let multiply = |x: u32| {
        println!("Multiplying");
        x * 2
    };

    let mut b = memoize(multiply);

    for i in 1..10 {
        println!("{}", b(i));
    }
    for i in 1..10 {
        println!("{}", b(i));
    }
}
