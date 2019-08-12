/**
- For data types stored on the heap, memory needs to be requested
  from the OS at runtime.
- The requested memory needs to be returned to the OS once we are
  done with it.
- In Rust, memory is automatically returned once the variable
  goes out of scope.

- A String data type has a pointer to the value, length & capacity;
  The value itself is stored on the heap but the metadata is on the
  stack.
- If s1 and s2 both point to the same value in memory, and they go
  out of scope, both of them would try to free the same memory - 
  double free error (memory safety bug)
*/

fn main() {
    // "String" - keyword here
    // "::" - operator that allows us to namespace the function under the String type
    // The String type is allocated on the heap and is able to store
    // an amount of text that is unknown to us at compile time
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    // as soon as s1 is moved to s2, s1 goes out of scope
    let s2 = s1;

    println!("{}, world!", s1);
}
