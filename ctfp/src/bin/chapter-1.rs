fn id<T>(x: T) -> T {
    x
}

fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let x = 1;
    let y = id(x);

    println!("x = {}, y = {}", x, y);

    let a = String::from("Hello");
    println!("a = {}", a);

    let b = id(a);
    println!("b = {}", b);

    let multiply_and_add = compose(|x: u32| x * 2, |x| x + 2);
    let divide_and_subtract = compose(|x: u32| x / 2, |x| x - 2);

    let res = compose(multiply_and_add, divide_and_subtract)(2);
    println!("res 2 *2 +2 /2 -2 = {}", res);

    let multiply_and_add = compose(|x: u32| x * 2, |x| x + 2);
    let l_id = compose(multiply_and_add, id)(2);

    let multiply_and_add = compose(|x: u32| x * 2, |x| x + 2);
    let r_id = compose(id, multiply_and_add)(2);

    println!("l_id = {}, r_id = {}", l_id, r_id);
}
