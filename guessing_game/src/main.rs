// use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Enter a number to guess? ");

    let random_number = rand::thread_rng().gen_range(1..=100);

    // println!("{}",random_number);

    
    
    loop{
        
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Expected to read line");
        let value:u32= match value.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("you guessed {}", value);
        
        match value.cmp(&random_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
        },
        }
    }
}
