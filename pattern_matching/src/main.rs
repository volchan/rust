fn main() {
    for i in 0..15 {
        println!("{}. I have {} oranges.", i, get_oranges(i));
    }

    // let point = (0, 0);
    // let point = (10, 0);
    // let point = (0, 10);
    let point = (10, 10);

    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis ({}, 0)", y),
        (x, y) => println!("x and y axis ({}, {})", x, y),
    }
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amout of",
        _ => "lots of",
    };
}
