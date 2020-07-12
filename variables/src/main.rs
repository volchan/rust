#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    // variables are immutable
    let name = "Alex"; // String
    let mut age = 32; // i32 mutable integer
    println!("{}", age);
    age = 42;
    println!("{}", age);

    let amount: i64 = 9987654321; // i64 integer for large numbers
    // Variables can be shadowed
    let color = "blue";
    let color = 86;
    println!("{}", color);

    // Declare multiple variables at once
    let (a, b, c) = (43, 85, "red");
    println!("a: {}, b: {}, c: {}", a, b, c);
}
