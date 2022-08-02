//call io lib for program input/output
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //print title message and ask for input with macro
    println!("~~~~~ Guess the number ~~~~~");

    //generate a secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is: {secret_number}");

    println!("Please input your guess");

    //create a new mutable string var called guess
    let mut guess = String::new();

    //read a line and store to guess, throw an error if necessary
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //output the guess with message
    println!("Your guess was: {guess}");

    //compare and match secretnumber to the below outcomes
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    
}
