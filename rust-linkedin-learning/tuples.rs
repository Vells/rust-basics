fn main() {
    let tup: (i32, f64, u8) = (500, 3.5, 1);

    // destructuring
    let (x, y, z) = tup;
    // OR
    let a = tup.0;
    let b = tup.1;

    println!("X = {}; Y = {}; Z = {}", x, y, z);
    println!("A = {}; B = {}", a, b);
}