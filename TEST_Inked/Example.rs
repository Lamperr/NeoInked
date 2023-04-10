use std::io;  // Import a crate

fn main() {

    println("Hello");
    // Declare variables with different types
    let mut x: i32 = 5;
    let y: f64 = 3.14;
    let z: bool = true;
    let s: &str = "hello";

    // Declare constants
    const PI: f64 = 3.14159265359;

    // Declare arrays and vectors
    let a: [i32; 3] = [1, 2, 3];
    let v: Vec<i32> = vec![4, 5, 6];

    // Declare tuples
    let t: (i32, f64, bool) = (7, 2.71, false);

    // Use conditional statements
    if x < 10 {
        println!("x is less than 10");
    } else if x == 10 {
        println!("x is equal to 10");
    } else {
        println!("x is greater than 10");
    }

    // Use loops
    for i in 0..3 {
        println!("a[{}] = {}", i, a[i]);
    }

    let mut i = 0;
    while i < v.len() {
        println!("v[{}] = {}", i, v[i]);
        i += 1;
    }

    // Declare functions with different syntax
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let subtract = |x: i32, y: i32| -> i32 { x - y };

    fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }

    let divide = |x: i32, y: i32| -> i32 { x / y };

    // Use try-except block
    let result = divide(10, 0);
    match result {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }

    // Declare a struct and an enum
    struct Person {
        name: String,
        age: i32,
    }

    enum Color {
        Red,
        Green,
        Blue,
    }

    // Declare a match statement
    let color = Color::Red;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    // Use other keywords
    let c = x + y as i32;
    let d = x * y as i32;
    let e = x % y as i32;
    let f = x.pow(y as u32);
    let g = x < y as i32;
    let h = x | y as i32;
    let i = x & y as i32;
    let j = !z;
    let k = PI;
    let l = s.len();
    let m = None::<i32>;
    let n: Result<i32, io::Error> = Ok(42);
    let o = v.is_empty();
    let p = s.chars().nth(0).unwrap_or('-');
}
