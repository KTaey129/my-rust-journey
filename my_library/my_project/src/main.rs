use my_library::{add, subtract, Point};

fn main() {
    println!("{}", add(3, 5));

    println!("{}", subtract(8, 5));

    let point = Point::new(3, 4);
    println!("{}", Point::distance_from_origin(&point));
}
