pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
    
    pub fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}