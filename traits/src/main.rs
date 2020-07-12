// Traits are like an interface or abstract class

struct RustDev {
    awesome: bool,
}

struct RubyDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello world!")
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello world!\");");
    }
}

impl Developer for RubyDev {
    fn new(awesome: bool) -> Self {
        RubyDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Ruby 2.7.0"
    }

    fn say_hello(&self) {
        println!("p \"Hello world!\"");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Java 1.8"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello world!\");");
    }
}

fn main() {
    // let r = RustDev { awesome: true };
    let rust_dev = RustDev::new(true);
    let ruby_dev = RubyDev::new(true);
    let java_dev = JavaDev::new(false);

    println!("{} :", rust_dev.language());
    rust_dev.say_hello();

    println!("{} :", ruby_dev.language());
    ruby_dev.say_hello();

    println!("{} :", java_dev.language());
    java_dev.say_hello();
}
