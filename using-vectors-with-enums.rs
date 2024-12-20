#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

// returns largest shape based on its area among the shapes in vector
fn largest_shape(shapes: &Vec<Shape>) -> &Shape {
    shapes
        .iter()
        .max_by(|a, b| {
            let area_a = match a {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Square(length) => length * length,
                Shape::Triangle(base, height) => base * height * 0.5,
            };
            let area_b = match b {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Square(length) => length * length,
                Shape::Triangle(base, height) => base * height * 0.5,

            };
            /* partial_cmp: Compares two floating-point numbers, 
                returning an Option<Ordering> (either Less, Equal, or Greater).
                Why Use total_cmp Instead of partial_cmp?
                    partial_cmp: Returns an Option<Ordering>:
                    Some(Ordering) for valid comparisons.
                    None for undefined comparisons (e.g., if one of the numbers is NaN).
                    total_cmp: Always returns a valid Ordering. This makes it more suitable for sorting or 
                    other operations that require a total order.*/
            area_a.partial_cmp(&area_b).unwrap()

        })
        .unwrap()

}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(5.0, 7.5)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => base * height / 2.0,
        })
        .sum();

    println!("Total area: {:.4?} sq. units", total_area);
    println!("the largest shape is {:?}", largest_shape(&shapes));
}
