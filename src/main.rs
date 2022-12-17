use std::{io};
use rand::Rng;

#[allow(non_snake_case)]

fn main() {
    let lowest_num: i32 = 1;
    let highest_num: i32 = 10;

    clear_console();
    println!("----- Guessing Game ----- ");

    loop {
        let mut some_input:String = String::new();
        let random_number: i32 = generate_rand(lowest_num, highest_num);

        println!("Please guess a number between {} and {}: ", lowest_num, highest_num);

        io::stdin()
            .read_line(&mut some_input)
            .expect("failed to read from stdin");

        let trimmed = some_input.trim();
        match trimmed.parse::<u32>() {
            Ok(_i) => process_input(
                some_input.trim().parse().expect("numeric"), random_number
            ),
            Err(..) => {
                if trimmed.to_lowercase() == "q" {
                    clear_console();
                    std::process::exit(0);
                } else if trimmed.to_lowercase() == "c" {
                    clear_console();
                } else {
                    println!("Not acceptable input. Options: \n {} - {} numeric guess \n q: Quit. \n c: Clear screen. ", lowest_num, highest_num)
                }
            }
        }
    }

}

fn generate_rand(in_low: i32, in_high: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let some_number: i32 = rng.gen_range(in_low..in_high);
    return some_number;
}

fn process_input(user_input: i32, random_input: i32) {
    if user_input == random_input {
        println!("Correct! High five! {}", random_input)
    } else {
        println!("Too bad. {}", random_input)
    }
}

fn clear_console() {
    //Just wanted to give it a better name. 
    print!("\x1B[2J\x1B[1;1H"); //Clears the console and moves the cursor to the top.
}

