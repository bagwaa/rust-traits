use std::marker::Copy;
use std::ops::Mul;

struct Square<T> {
    x: T,
    y: T,
}

impl<T: Mul<Output = T> + Copy> Square<T> {
    fn new(x: T, y: T) -> Square<T> {
        Square { x, y }
    }

    fn area(&self) -> T {
        self.x * self.y
    }
}

fn main() {
    let square = Square::new(10.1, 10.2);
    println!(
        "The square is {} cm by {} cm and the area is {} cm",
        square.x,
        square.y,
        square.area()
    );

    let square = Square::new(10, 12);
    println!(
        "The square is {} cm by {} cm and the area is {} cm",
        square.x,
        square.y,
        square.area()
    );
}

// improvements, I would like to be able to pass in any valid positive number into the arguments
// when creating a square or a circle, casting everything to f64 feels a bit meh
