trait Duplicable {
    fn dupl(&self) -> String;
}

impl Duplicable for String {
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl Duplicable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

// fn duplicate<T: Duplicable> (x: T) {
//     println!("{}", x.dupl())
// }

fn duplicate(x: &dyn Duplicable) {
    println!("{}", x.dupl())
}

fn main() {
    let a = 42;
    let b = "Hi John ".to_string();

    // duplicate(a);
    // duplicate(b);

    duplicate(&a);
    duplicate(&b);
}
