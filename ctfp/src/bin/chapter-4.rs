fn compose<A, F, G>(f: F, g: G) -> impl Fn(A) -> Option<A>
where
    F: Fn(A) -> Option<A>,
    G: Fn(A) -> Option<A>,
{
    move |x| {
        if let Some(x) = f(x) {
            g(x)
        } else {
            None
        }
    }
}

fn safe_root(a: f64) -> Option<f64> {
    if a >= 0.0 {
        Some(a.sqrt())
    } else {
        None
    }
}

fn safe_divide(a: f64) -> Option<f64> {
    if a != 0.0 {
        Some(6.0 / a)
    } else {
        None
    }
}

fn safe_reciprocal(a: f64) -> Option<f64> {
    if a != 0.0 {
        Some(1.0 / a)
    } else {
        None
    }
}

fn main() {
    let a = safe_root(4.0);
    println!("a = {:?}", a);

    let b = safe_root(-1.0);
    println!("b = {:?}", b);

    let c = safe_divide(2.0);
    println!("c = {:?}", c);

    let d = safe_divide(0.0);
    println!("d = {:?}", d);

    let safe_root_reciprocal = compose(safe_root, safe_reciprocal);
    println!("e = {:?}", safe_root_reciprocal(4.0));
}
