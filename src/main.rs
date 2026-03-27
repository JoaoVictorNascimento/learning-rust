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

    // shadowing
    let a = 5;

    let a = a + 1;

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a is: {a}");

    // data types
    // scalar types
    // integer types
    let x: i32 = 10;
    println!("x is {x}");

    // floating-point types
    let y: f64 = 10.0;
    println!("y is {y}");
    
    // boolean types
    let z: bool = true;
    println!("z is {z}");

    // character types
    let c: char = 'a';
    println!("c is {c}");
    
    
    // compound types
    // tuple types
    let tup: (i32, f64, bool, char) = (10, 10.0, true, 'a');
    println!("tup is {tup}");

    // array types
    let arr: [i32; 3] = [1, 2, 3];
    println!("arr is {arr}");
    
    
}
