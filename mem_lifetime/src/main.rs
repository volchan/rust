#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person,
}

impl Person {
    fn get_name(&self) -> &String {
    // fn get_name<'l> (&'l self) -> &'l String { // Same as above
        &self.name
    }
}

fn main() {
    println!("{}", get_str());

    let p1 = Person { name: String::from("John") };
    let d1 = Dog { name: String::from("Max"), owner: &p1 };

    println!("{:?}", d1);

    let mut a: String;
    {
        let p2 = Person { name: String::from("Mary") };
        // a = p2.get_name(); // Error: p2 will not live after the end of this scope
        a = p1.get_name();
    }
    println!("{}", a);
}

fn get_str() -> &'static str {
    "Hello"
}
