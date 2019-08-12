fn main() {
    let refers_nothing = dangle();
    println!("{}", refers_nothing)
}


// we are trying to create a race condition
// between the function clearing out its memory from the heap
// and the main function trying to access this particular reference
fn dangle() -> String {
    let s = String::from("hello");
    s
    // rust will not let us return a reference to this string
    // because the string will no longer exist once
    // the function returns;
    // the function needs to return the string, not a reference to it
}
