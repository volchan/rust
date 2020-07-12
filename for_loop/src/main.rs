fn main() {
    for i in 1..11 {
        println!("{0} * {0} = {1}", i, i * i);
    }

    let pets = ["cat", "dog", "chihuahua", "bear", "hamster"];
    for pet in pets.iter() {
        if pet == &"chihuahua" {
            println!("{} barks too much", pet);
            continue // Skips to the next iteration
        }
        if pet == &"bear" {
            println!("{} is not a pet", pet);
            break // Stops the loop
        }
        println!("I love my {}.", pet);
    }

    for (pos, i) in (1..11).enumerate() {
        let square = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1}", nb, square);
    }
}
