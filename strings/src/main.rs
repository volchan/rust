#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    let cat: &'static str = "Fluffy";
    println!("{}", cat);

    let dog = String::new();
    let mut dog = String::from("Max");
    println!("{}", dog);

    let owner = format!("Hi i'm {}, the owner of {}.", "Mark", dog);
    println!("{}", owner);

    println!("{}", dog.len());

    dog.push(' '); // Can only push 1 char
    dog.push_str("the dog."); // Can push any strings
    println!("{}", dog);

    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog);
}
