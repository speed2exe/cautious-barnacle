fn main() {
    println!("Hello, world!");
    const x2: i32 = 5;
    // const x = 5; -> not allowed
    const x1: u32 = 3 * 5;
    // const z: i64 = get_num(); -> not allowed

    let z: i64 = 6;
    let z = "hello"; // -> shadows the previous z


    // if u32 is removed, complier will complain because it does not know that you want to parse
    // u32
    let guess: u32 = "42".parse().expect("Not a number!");

    // usize and usize depends on architecture (32b or 64b)
    // The primary situation in which you’d use isize or usize is when indexing some sort of collection.

    // char is 4 bytes
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';


    // Compound types

    // tuple
    // like struct but nameless and without impl
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Access tuple elements
    let (x, y, z) = tup; // destructuring
    // acccess tuple elements by index
    let x: i32 = tup.0; // dot notation

    let unit = (); // unit type
    // function that returns unit if no return value

    // ARRAYS
    // every element has the same type
    // useful for fixed-size collections allocated on the stack
    // not as flexible as vectors, cannot be resized
    // useful and fast if you know the size in advance
    let a = [1, 2, 3, 4, 5]; // array of 5 elements
    // Access array elements with index notation
    // Will panic if index is out of bounds (do your checks)
    a[1]; 
    
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 elements with type annotation
    let a = [3; 5]; // array of 5 elements with value 3
    let a: [i32; 5]; // array of 5 elements with type annotation
    // initializing array later (higher performance, but unsafe)
    let mut a: [i32; 5] = unsafe { std::mem::uninitialized() }; // uninitialized array, garbage value
    
    // variable names are snake_case
    let my_num = 5;

    // statements do not return a value
    // let x = (let y = 6); // let is not allowed

    // A new scope is created for with curly braces is an expression
    // expression do not include semicolons, unlike statements
    let x = {
        let k = 2;
        k * 2
    };

    // Control Flow
    // if, else, else if
    if x == 5 {
        println!("x is 5");
    } else if x == 6 {
        println!("x is 6");
    } else {
        println!("x is neither 5 nor 6");
    }

    // Using if in a let statement
    let y = if x == 5 { 10 } else { 15 };

    let y = {
        if x == 5 { 
            10 
        } else {
            15 
        }
    };

    // loop
    // again is printed non stop
    loop {
        println!("again!");
        // break; // use break to exit loop
    }

    // returning values from loop
    // break is used to exit loop, 
    let x: i32 = loop {
        break 2;
    };

    // conditional loop with while
    let mut x = 5;
    while x != 0 {
        println!("{}!", x);
        x -= 1;
    }

    // looping through a collection
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let a = 1..4;
    for num in a {
        println!("{}!", num);
    }

    for i in a.rev() {
        println!("{}!", i);
    }
    
    
}

// function name are snake_case
// last unassigned expression is returned from function
fn get_num() -> i64 {
    54
}

// function parameters type must be declared
fn add_one(x: i32) -> i32 {
    x + 1
}

// Difference between let and const
// const are ALWAYS immutable, type must be annotated
//
