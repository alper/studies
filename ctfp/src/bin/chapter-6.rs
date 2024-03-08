// 1. Show the isomorphism between Maybe a and Either () a

// maybeToEither :: Maybe a -> Either () a
// maybeToEither Maybe a = Either () a

// eitherToMaybe :: Either () a -> Maybe a
// eitherToMaybe (Left _) = Nothing
// eitherToMaybe (Right a) = Just a

// 2. Here’s a sum type defined in Haskell

pub trait Shape {
    fn area(&self) -> f64;
    fn circ(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }

    fn circ(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

pub struct Rect {
    length: f64,
    width: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.length * self.width
    }

    fn circ(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
}
// 4. Add a new shape, Square, to Shape and make all the necessary updates.
pub struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn circ(&self) -> f64 {
        4.0 * self.side
    }
}

// 5. Show that a + a = 2 * a holds for types

// Either a a = (Bool, a)

// sumToPair :: Either a a -> (Bool, a)
// sumToPair Left a = (True, a)
// sumToPair Right a = (False, a)

// pairToSum :: (Bool, a) -> Either a a
// pairToSum (True, a) = Left a
// pairToSum (False, a) = Right a

fn main() {
    let c = Circle { radius: 4.0 };
    let r = Rect {
        length: 2.0,
        width: 3.0,
    };
    let s = Square { side: 2.0 };

    println!("Cicle = area: {} circumference: {}", c.area(), c.circ());
    println!("Rect = area: {} circumference: {}", r.area(), r.circ());
    println!("Square = area: {} circumference: {}", s.area(), s.circ());
}
