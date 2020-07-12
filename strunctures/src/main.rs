fn main() {
    // It's like a class

    let emp = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35,
    };

    println!("{:?}", emp);
    println!("{}", emp.name);
    println!("{}", emp.fn_details());
    println!("{}", Employee::static_fn_detail());
}

#[derive(Debug)] // Need this to be able to print a struct
struct Employee {
    name: String,
    company: String,
    age: u32,
}

impl Employee {
    fn fn_details(&self) -> String {
        format!(
            "name: {}, age: {}, company: {}",
            &self.name, &self.age, &self.company
        )
    }

    fn static_fn_detail() -> String {
        String::from("Details of a person")
    }
}
