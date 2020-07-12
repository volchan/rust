// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;
use rand::Rng;

mod archive;

fn main() {
    // arch_file("somefile.txt");
    arc("somefile.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
}
