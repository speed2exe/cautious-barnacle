fn main() {
    println!("Hello, world!");

    let s: &str = "hello 世界";
    print_type_of(&s);
    // print address of s
    println!("{:p}", s);
    // print chars of s
    for c in s.chars() {
        println!("char: {}", c);
    }
    // print bytes of s
    for b in s.bytes() {
        println!("byte: {}", b);
    }
    s[0];


    // OK because s is a &str and content is stored in heap
    // content s is not resizable
    let s2 = s;
    // print address of s2
    println!("{:p}", s2);
    // print chars of s2
    for c in s2.chars() {
        println!("char: {}", c);
    }
    // print bytes of s
    for b in s2.bytes() {
        println!("byte: {}", b);
    }


    // dynamic string, can be resied
    let mut s = String::from("hello");
    // append s to s
    s.push_str(" world"); // s need to be mutable
    print_type_of(&s);

    // print s
    println!("{}", s);

    // print memory address of s
    println!("addr: {:p}", &s);

    let s2 = s;
    // s is not valid anymore, ownership transferred to s2
    print_type_of(&s2);
    // print s2
    println!("{}", s2);

    // print memory address of s2
    // observe memory address of s2 is same as s
    println!("addr: {:p}", &s2);

    // cannot use s anymore because it's moved
    // println!("{}", s);

    // clone s2 to s3
    let s3 = s2.clone();
    // print s3 memory address
    println!("addr: {:p}", &s3);

    // print s2
    // s2 still valid because s3 is cloned, mem address is different
    println!("{}", s2);


    takes_ownership(s2);
    // cannot use s2 anymore because it's moved
    // println!("{}", s2);

    let mut s = String::from("hello");
    takes_reference(&s);
    println!("{}", s); // still valid because owner of s is not change
    takes_mutable_reference(&mut s);
    println!("{}", s); // still valid because owner of s is not change

    let mut s = String::from("hello");
    let r1 = &mut s;
    // not allowed to have more than 2 mutable simultaneous reference existing
    // let r2 = &mut s; 
    // println!("{}, {}", r1, r2);
  
    { // confined in scope with curly
        let mut s = String::from("hello");
        let r1 = &s;
        // not allowed to have both mutable and immutable reference existing
        // either (1 or more immutable ref) or (1 mutable ref)
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
    }
    {
        let a: [u8; 3] = [1,2,3];
        let b = &a;
    }
    let x = 5;
    takes_ownership_int(x);
    // still valid because x is a type that impl the Copy trait
    println!("{}", x);
    // types that impl Copy trait are copied when passed to a function
    // or assigned to a variable, below are examples of types that impl Copy trait
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

{
        let s = String::from("hello world");
        let hello = sub_string(&s);
        let world = &s[6..11];

    }


}

fn sub_string(s: &String) -> &str {
    &s[0..5]
}

fn takes_ownership(some_string: String) {
    println!("taken ownersip of: {}", some_string);
}

fn takes_reference(some_string: &String) {
    println!("taken ownersip of: {}", some_string);
}

fn takes_mutable_reference(some_string: &mut String) {
    println!("taken mutable ref: {}", some_string);
    some_string.push_str(" world"); // s need to be mutable
}

fn takes_ownership_int(some_int: i32) {
    println!("taken ownersip of: {}", some_int);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// not allowed to return dangling pointers
// returns dangling reference
// fn dangling_reference() -> &String {
//     let s = String::from("hello");
//     &s
// }
