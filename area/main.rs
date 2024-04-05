use std::fmt::Debug;

trait Area: Debug {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.141592653589793238 * self.radius * self.radius
    }
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

#[derive(Debug)]
struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Area>(shape: T) {
    println!("The area of the shape is: {:?}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 4.0, height: 6.0 };
    let square = Square { side: 3.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}
