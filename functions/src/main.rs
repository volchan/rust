#![allow(unused_variables)]
#![allow(unused_assignments)]

// fn main() {
//     let name = "John";
//     say_hi(name);
// }

// fn say_hi(name: &str) {
//     println!("Hello {}!", name);
// }

// fn main() {
//     let mut name = "John";
//     say_hi(&mut name);
//     println!("{}", name);
// }

// fn say_hi(name: &mut &str) {
//     println!("Hello {}!", name);
//     *name = "Alex";
// }

fn main() {
    let name = "John";
    println!("{}", say_hi(name));
}

fn say_hi(name: &str) -> String {
    return format!("Hello {}!", name);
}
