use std::io;
use rand::Rng;

fn main() {
    println!("Enter a number to guess? ");

    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("{}",random_number);

    let mut value = String::new();

    io::stdin().read_line(&mut value).expect("Expected to read line");

    println!("you guessed {}", value);

    if random_number.to_string() == value {
        println!("Correct!");
    }
    else {
        println!("incorrect!");
    }
}
