// takes a value and a vector, put the value in the vector at 1st place and last place
fn insert_value(v: &mut Vec<i32>, value: i32) {
    v.insert(0, value);
    v.push(value);
} 

// append a vector to another vector
fn append_vector(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    v2.append(v1);
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    // put a value to the vector at 1st and last place
    let value:i32 = 15;
    insert_value(&mut v, value);
    println!("{:?}", v);

    // append a vector to another vector
    let mut v2: Vec<i32> = vec![20, 30, 40];
    append_vector(&mut v2, &mut v);
    println!("{:?}", v);
    println!("{:?}", v2);
}
