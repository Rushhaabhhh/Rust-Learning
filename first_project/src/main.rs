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
    let (_x, y, _z) = tup;
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

    // Functions
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    println!("The sum of 5 and 6 is {}", add(5, 6));

    // Memory Management

        // In java, javascript, memory is managed by garbage collector where data is allocated and deallocated automatically. It is safe but slow.
        // In C, C++, memory is managed by programmer where data is allocated and deallocated manually. It is prone to memory leaks and dangling pointers
        // In rust, memory is managed by compiler where data is allocated and deallocated automatically. It is safe and fast.
        // Rust achieves this by using Mutability, Ownership, Borrowing, Lifetimes, and Heap.

    // Stack and Heap

        // Rust stores data in stack and heap. In stack non-fixed size data is stored and in heap fixed size data is stored. Stack is faster than heap. 
        // Stack is managed by compiler and heap is managed by OS.
        // Stack : Variables, Functions, Pointers, etc.
        // Heap : Strings, Vectors, etc.

    // Ownership

        // Each peice of data in Rust has a single owner. There can only be one owner at a time. When the owner goes out of scope, Rust automatically deallocates the data, preventing memory leaks.
}
