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
    let mut a = 5;
    a = 6;
    println!("a is {}", a);

    let is_bool: bool = true;
    println!("Boolean value is {}", is_bool);

    // Strings are stored in heap
    let str = "Hello, world!";
    println!("{}", str);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    // Arrays
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("Array element at index 2 is: {}", arr[2]);

    // Conditionals
    let condition = true;
    if condition {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    // Loops
    let mut counter = 0;
    for _i in 0..10 { // _i is a throwaway variable, therefore ' _ ' is used
        counter += 1;
        println!("Counter is {}", counter);
    }

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("Liftoff!");

    // Functions
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    println!("The sum of 5 and 6 is {}", add(5, 6));

    // Memory Management

    // Ownership
    let s = String::from("hello");
    takes_ownership(s);
    
    let x = 5;
    makes_copy(x);

    // Borrowing
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("Changed string: {}", s1);

    // Vectors
    let mut v = vec![1, 2, 3];
    v.push(4);
    for i in &v {
        println!("Vector element: {}", i);
    }

    // Structs
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle area: {}", area(&rect1));

    // Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Write(String::from("Hello"));
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}

fn takes_ownership(some_string: String) {
    println!("String taken: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Integer copied: {}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

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
