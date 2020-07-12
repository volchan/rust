fn main() {
    // Static - can't be resized
    // Elements can be updated
    // Indexed
    // Limited to 12 elements

    // let person = ("John", 27, true);
    let mut person: (&str, i32, bool) = ("John", 27, true);
    println!("{:?}", person);
    println!("{}", person.0);

    person.0 = "Mike";
    println!("{:?}", person);
    println!("{}", person.0);

    let (name, age, employment) = person;
    println!("name: {}, age: {}, employment: {}", name, age, employment);
}
