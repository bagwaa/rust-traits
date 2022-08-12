use std::fmt::Display;
use std::marker::Copy;
use std::ops::{Add, Mul};

struct Square<T> {
    x: T,
    y: T,
}

impl<T: Display + Mul<Output = T> + Add<Output = T> + Copy> Square<T> {
    fn new(x: T, y: T) -> Square<T> {
        Square { x, y }
    }

    fn area(&self) -> T {
        self.x * self.y
    }

    fn parameter(&self) -> T {
        self.x + self.y + self.x + self.y
    }

    fn log(&self) {
        println!(
            "The square is {} cm by {} cm and the area is {} cm",
            self.x,
            self.y,
            self.area()
        );
        println!("The parameter of sqaure is {:.2} cm", self.parameter());
    }
}

fn main() {
    let square = Square::new(10.1, 10.2);
    square.log();

    let square = Square::new(10, 12);
    square.log();
}
