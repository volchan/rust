// Closures are anonymous functions/lambdas
// Can be assigned to a variable
// Can be generic, but only accept the first type it was called with

fn main() {
    let a = |a: i32| a + 1;
    println!("{}", a(6));
    
    let b = |b: i32| -> i32 {
        let c = b + 1;
        c
    };
    println!("{}", b(4));

    let gen = |x| println!("{}", x);
    gen(3);
    // gen(true); // Error if called after line 17 but not if called alone
}
