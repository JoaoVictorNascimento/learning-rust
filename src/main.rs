/// This is a documentation comment
fn main() {
    // single line comment
    /*
        multi line comment
    */
    println!("Hello, world!");

    // immutable variable
    let y = 10;
    println!("y is {y}");

    // mutable variable
    let mut x = 10;
    println!("x is {x}");

    x = 20;
    println!("x is {x}");

    // constant variable
    const V: u32 = 10;
    println!("V is {V}");
}
