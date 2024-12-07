fn main() {
    println!("Advanced Rust Concepts!");

    // Error Handling with Result and Option
    let result = divide(10, 2);
    match result {
        Ok(quotient) => println!("Quotient is: {}", quotient),
        Err(e) => println!("Error: {}", e),
    }

    let option = get_element(&[1, 2, 3], 1);
    match option {
        Some(value) => println!("Found element: {}", value),
        None => println!("Element not found"),
    }

    // Iterators and Closures
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    let filtered: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Filtered even numbers: {:?}", filtered);

    // Traits and Generics
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };
    println!("Integer Point: {:?}", p1);
    println!("Float Point: {:?}", p2);

    let rectangle = Rectangle { width: 30, height: 50 };
    println!("Rectangle area: {}", rectangle.area());

    // Smart Pointers
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("List: {:?}", list);

    // Concurrency
    use std::thread;
    use std::sync::Arc;

    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread sees: {:?}", data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Async/Await
    futures_executor::block_on(async_main());
}

// Function for error handling
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn get_element(array: &[i32], index: usize) -> Option<i32> {
    array.get(index).copied()
}

// Struct and Trait Example
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Enum with Smart Pointers
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Async/Await Example
async fn async_main() {
    let result = async_work().await;
    println!("Async result: {}", result);
}

async fn async_work() -> i32 {
    42
}
