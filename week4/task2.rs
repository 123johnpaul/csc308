#[derive (Debug)]
struct Circle {
    radius: f32,
}

use std::io;
impl Circle {
    fn area(&self) -> f32 {
        3.14 * (self.radius * self.radius)
    }


    fn circumfrence(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
}
fn main() {
    println!("Input radius of circle:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input: f32 = user_input.trim().parse().expect("Please type a number!");
    let circle1 = Circle {
        radius: user_input,
    };
    println!("Area of circle: {}", circle1.area());
    println!("Circumfrence of circle: {}", circle1.circumfrence());
}

