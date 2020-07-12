use crate::Colors::Red;
use crate::Person::Name;

fn main() {
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    
    let my_color = Red;
    println!("{:?}", my_color);

    let person = Name(String::from("Alex"));
    println!("{:?}", person);
}

#[derive(Debug)] // Need this to print enums
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)] // Need this to print enums
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
}