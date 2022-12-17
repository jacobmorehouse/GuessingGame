use std::io;
use rand::Rng;

#[allow(non_snake_case)]

fn main() {
    let lowest_num: i32 = 1;
    let highest_num: i32 = 10;

    //TODO:  make menu to exit loop

    //TODO: clear the console each time so that it looks cleaner.


    println!("----- Guessing Game ----- ");

    loop {
        let mut some_input:String = String::new();
        let mut random_number: i32 = generate_rand(lowest_num, highest_num);


        println!("Please guess a number between {} and {}: ", lowest_num, highest_num);

        io::stdin()
            .read_line(&mut some_input)
            .expect("failed to read from stdin");

        let trimmed = some_input.trim();
        match trimmed.parse::<u32>() {
            Ok(_i) => process_input(
                some_input.trim().parse().expect("numeric"), random_number
            ),
            Err(..) => println!("This was not an integer: {}", trimmed),
        };

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

