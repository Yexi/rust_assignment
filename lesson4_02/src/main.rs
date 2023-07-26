use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let newPoint = Point{ x: 0, y: 2} + Point{ x: 1, y: 3 };
    println!("{:?}", newPoint);
}
