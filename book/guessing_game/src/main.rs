use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

// The main function
fn main() {
    // random number from 1 to 10
    let secret_number = rand::thread_rng().gen_range(0..10);

    loop {
        match get_guess().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you got it!");
                return;
            }
        }
    }
}

// get guess from user
fn get_guess() -> i64 {
    println!("Please input your guess.");

    let mut input = String::new();

    loop {
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        match input.trim().parse::<i64>() {
            Ok(num) => {
                return num;
            }
            Err(err) => {
                println!("Please input a number, err: {}", err);
                input.clear();
            },
        }
    }
}
