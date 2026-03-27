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
    println!("tup is {tup:?}");

    // array types
    let arr: [i32; 3] = [1, 2, 3];
    println!("arr is {arr:?}");

    // functions
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    println!("add(1, 2) is {}", add(1, 2));

    // control flow
    // if expression
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // loop
    let mut count = 0;
    loop {
        count += 1;
        println!("count is {count}");
        if count == 10 {
            break;
        }
    }

    // while loop
    let mut count = 0;
    while count < 10 {
        count += 1;
        println!("count is {count}");
    }

    // for loop
    let arr = [1, 2, 3, 4, 5];
    for element in arr {
        println!("element is {element}");
    }

    // match expression
    let number = 6;
    match number {
       1 => println!("one"),
       2 => println!("two"),
       3 => println!("three"),
       4 => println!("four"),
       5 => println!("five"),
       _ => println!("other"),
    }

    // if let expression    
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let expression with pattern
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
