use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn point_add<T: Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}

fn main() {
    let a = Point {
        x: 1.1,
        y: 2.2,
    };
    let b = Point {
        x: 3.3,
        y: 1.1,
    };
    let result = point_add(a, b);
    println!("{:?} + {:?} = {:?}", a, b, result);
}