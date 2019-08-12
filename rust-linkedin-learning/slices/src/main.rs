/*
- Slice is a reference to a part of a string
*/


fn main() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    println!("hello contains = {}", hello);
}
