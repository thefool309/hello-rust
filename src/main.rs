use std::io;

use rand::Rng;

fn main() { //comments work like c or c++
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("The Secret Number is : {}", secret_number);

    println!("You guessed: {}", guess);




}
