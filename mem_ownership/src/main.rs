fn main() {
    let i = 5;
    let j = i;
    println!("{}", i);
    println!("{}", j);

    let v = vec![1, 2, 3, 4, 5];
    // let w = v;
    // println!("{:?}", w);
    // println!("{:?}", v); // V is unavailable here because ownership has been transferred to w;

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };
    let v = foo(v);
    println!("{:?}", v);
}
