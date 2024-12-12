use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() { //comments work like c or c++
    let mut user_finished: bool = false;

    while !user_finished {

        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..100);

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("The Secret Number is : {}", secret_number);

        println!("You guessed: {}", guess);

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => println!("You win!")
        }

        println!("Continue Playing?(1 for yes 2 for no)");

        let mut answer: String = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line!");

        
        let answer: u32 = answer.trim().parse().expect("Please Type 1 or 2!");

        if answer == 2 {
            user_finished = true;
        }
    }
}
