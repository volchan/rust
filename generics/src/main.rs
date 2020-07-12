use crate::Colors::Red;

fn main() {
    let p1: Point<i32> = Point { x: 6, y: 8 };
    let p2: Point<f64> = Point { x: 3.25, y: 8.63 };
    println!("{:?}", p1);
    println!("{:?}", p2);
    
    let c1 = Red("#f00");
    let c2 = Red(255);
    println!("{:?}", c1);
    println!("{:?}", c2);
    
    let p3: Point2<i32, f64> = Point2 { x: 34, y: 8.5 };
    println!("{:?}", p3);
}

#[derive(Debug)] // Need this to print
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)] // Need this to print
enum Colors<T> {
    Red(T),
    Green(T),
    Blue(T),
}

#[derive(Debug)] // Need this to print
struct Point2<T, V> {
    x: T,
    y: V,
}
