// Macros = Meta programming
// Match an expression and perform some operation
// Code is expanded and compiled
// Macros can match: expr, ident, block, stmt, par, path, meta, ty, tt

macro_rules! my_macro {
    () => {
        println!("First macro")
    };
}

// macro_rules! name {
//     ($name: expr) => {
//         println!("Hey {}", $name);
//     };
// }

macro_rules! name {
    ($($name: expr), *) => {
        $(println!("Hey {}", $name);)*
    };
}

macro_rules! xy {
    (x => $e: expr) => (println!("x is {}", $e));
    (y => $e: expr) => (println!("y is {}", $e));
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called.", stringify!($fn_name));
        }
    }
}

fn main() {
    my_macro!();

    name!("John");
    name!("Alex", "Mary", "Carol");

    xy!(x => 5);
    xy!(y => 3 * 9);

    build_fn!(hey);
    hey();
}
