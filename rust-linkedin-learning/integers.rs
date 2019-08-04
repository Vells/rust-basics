fn main() {
    // this is the max number that a 32-bit int can store
    let mut a:i64 = 2147483647;

    // by adding 1, we try to overflow that max number that is being stored in a
    a = a + 1;
    println!("a = {}", a);
}
