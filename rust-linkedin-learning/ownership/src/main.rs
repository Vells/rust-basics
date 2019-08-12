/**
Ownership rules:
    1- Each value has a variable called its owner
    2- There can only be one owner at a time
    3- When the owner goes out of scope, the value is dropped
*/


fn main() {
    // scope is defined with curly braces;
    // if x was inside another set of curly braces,
    // it would have not been accessible from the outside the scope
    let x = 1;

    if x == 1 {
        let a = 10;
        println!("A = {}", a);
    }
}
