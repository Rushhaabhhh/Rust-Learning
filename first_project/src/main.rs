fn main() {
    println!("Hello, world!");

    // Primitive data types
    
    // Variables are immutable by default
    /* 
    i : signed integer
    u : unsigned integer
    f : floating point 
    bool : boolean
    char : character
    */ 
    let x: i32 = 5; 
    let y = x + 1;
    println!("x + 1 is {}", y);

    // Variables are mutable with "mut" keyword
    // let mut a = 5;
    // a = 6;
    // println!("a is {}", a);

    let is_bool: bool = true;
    print!("Boolean value is {}", is_bool);

    // Strings are stored in heap
    let str = "Hello, world!";
    println!("{}", str);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Conditionals
    let condition = true;
    if condition {
        println!("Condition is true");
    } else if !condition {
        println!("Condition is false");
    }

    // Loops
    let mut counter = 0;
    for _i in 0..10 { // _i is a throwaway variable, therefore ' _ ' is used
        counter += 1;
    println!("Counter is {}", counter);
    }
}
