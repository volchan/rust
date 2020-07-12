static mut R: i32 = 0; // Mutable variable declared in the global scope

fn main() {
    {
        let a = 3;
        println!("{}", a);
    }
    // println!("{}", a); // Error "a" is not defined in this scope

    unsafe { // Needed to use variables declared in the global scope
        R = 4;
        println!("{}", R);
    }

    unsafe { // Needed to use variables declared in the global scope
        println!("{}", R);
    }
}
