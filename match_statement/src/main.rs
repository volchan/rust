use crate::Suit::{Heart, Spade, Club, Diamond};

fn main() {
    print_choise(Heart);
    print_choise(Spade);
    print_choise(Club);
    print_choise(Diamond);

    country(44);
    country(34);
    country(125);
    country(-50);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

fn print_choise(choice: Suit) {
    // Similar to when/switch/case in other languages
    // Can return a result
    // Ranges are allowed
    match choice {
        Heart => { println!("\u{2665}") }
        Spade => { println!("\u{2660}") }
        Club => { println!("\u{2663}") }
        Diamond => { println!("\u{2666}") }
    }
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "Unknown",
        _ => "Invalid"
    };
    println!("Country is {}.", country);
}
