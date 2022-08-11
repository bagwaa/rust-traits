use std::ops::Mul;

struct Square<T> {
    x: T,
    y: T,
}

impl<T: Mul> Square<T> {
    fn new(x: T, y: T) -> Square<T> {
        Square { x, y }
    }

    fn area(&self) -> T {
        &self.x * &self.y
    }
}

fn main() {
    let first_square = Square::new(10, 10);
    println!(
        "The square is {} cm by {} cm and the area is {} cm",
        first_square.x,
        first_square.y,
        first_square.area()
    );
}
