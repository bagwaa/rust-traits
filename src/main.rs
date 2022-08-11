use std::f64::consts::PI;

// This is a bit like an interface in PHP, anything that implements this trait must provide an
// implementation for the area method.
trait HasArea {
    fn area(&self) -> f64;
}

// Define a simple struct for a Square with a length and a width
struct Square {
    x: f64,
    y: f64,
}

// Add a static method to the Square struct that we can use to construct a square
impl Square {
    fn new(x: f64, y: f64) -> Square {
        Square { x, y }
    }
}

// Implement the area() method for the Square
impl HasArea for Square {
    fn area(&self) -> f64 {
        self.x * self.y
    }
}

// Define a simple struct for a Circle with just a radius
struct Circle {
    r: f64,
}

// Add a static method to the Circle struct that we can use to construct a circle
impl Circle {
    fn new(r: f64) -> Circle {
        Circle { r }
    }
}

// Implement the area() method for the Circle
impl HasArea for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }
}

fn main() {
    let square = Square::new(10_f64, 10_f64);
    println!(
        "The square is {} cm by {} cm and the area is {} cm",
        square.x,
        square.y,
        square.area()
    );

    let circle = Circle::new(10_f64);
    println!(
        "The circle has a radius of {} cm and an area of {:.2} cm",
        circle.r,
        circle.area()
    );
}

// improvements, I would like to be able to pass in any valid positive number into the arguments
// when creating a square or a circle, casting everything to f64 feels a bit meh
