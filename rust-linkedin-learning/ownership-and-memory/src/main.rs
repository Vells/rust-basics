fn main() {
    // s comes into scope; this comes from heap
    let mut s = String::from("hello");

    // s's value moves into the function...
    // ... and so is no longer valid here;
    // but if we add &, the value will be passed by reference
    // and the function will not take ownership
    takes_ownership(&mut s);             

    println!("{}", s);
    // x comes into scope; this comes from stack
    let x = 5;

    // x would move into the function,
    makes_copy(x)
    // but i32 is Copy, so it’s okay to still
    // use x afterward.

}
// Here, x goes out of scope, then s. But since s's value was moved, nothing
// special happens.

// some_string comes into scope.
fn takes_ownership(some_string: &mut String) {
    println!("{}", some_string);
    some_string.push_str(" world!");
}
// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

// some_integer comes into scope.
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
// Here, some_integer goes out of scope. Nothing special happens.
