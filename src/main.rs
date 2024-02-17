use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("***************************************************************");
    println!("****Tahiat's super advanced rust number guessing game 2024!****");
    println!("***************************************************************");
    println!("******************* _______  _______ **************************");
    println!("*******************|       ||       |**************************");
    println!("*******************|_     _||    ___|**************************");
    println!("*******************  |   |  |   | __ **************************");
    println!("*******************  |   |  |   ||  |**************************");
    println!("*******************  |   |  |   |_| |**************************");
    println!("*******************  |___|  |_______|**************************");
    println!("***************************************************************");
    println!("***************************************************************");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_counter = 0;

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Uh oh! It is not a number");
                continue;
            },
        };
        
        guess_counter+= 1;
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            },
            Ordering::Greater => {    
                println!("Too big!");
            },
            Ordering::Equal => {
                println!("after {} guesses,", guess_counter);
                println!("You win!");
                break;
            },
        }
    }

}
