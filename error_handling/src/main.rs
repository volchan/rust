use std::fs::File;

fn main() {
    // let v = vec![1, 2, 3];
    // v[10];
    // panic!("Something went wrong. Cannot proceed");

    let f = File::open("main.jpg");
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        },
        Err(e) => {
            println!("file not found\n{:?}", e);
        }
    }
    println!("Code is still continuing.");

    divide(Some(1));
    divide(Some(10));
    divide(None);
    divide(Some(0));
}

const ANSWER_TO_LIFE: i32 = 42;

fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("cannot divide by 0"),
        Some(x) => println!("Result is {}", ANSWER_TO_LIFE / x),
        None => println!("None received, the answer is {}", ANSWER_TO_LIFE),
    }
}
