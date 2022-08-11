struct Square<T> {
    x: T,
    y: T,
}

impl<T> Square<T> {
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

// How do I remove the i32 here?  I want to use a Generic everywhere
// If I use a trait T it doesn't understand how to multiply the X and Y properties

// Once we have a generic type running correctly, I then want a trait with the area function inside

// and then implement that trait for a circle and a square
