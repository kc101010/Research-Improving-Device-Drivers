/*
Rust Calculator written by Kyle Christie 
14/8/22
Learning rust for Improving device drivers hons project @ UWS

*/

use std::io;

extern crate meval;

fn main() {
    //print program title
    println!("~~~~~ Calculator ~~~~~");

    //mutable string holds calculation from user
    let mut in_calculation = String::new();
    
    //ask for calculation and carry out 
    println!("Please enter your calculation");
    io::stdin()
    .read_line(&mut in_calculation)
    .expect("Failed to read line");

    let result = meval::eval_str(&in_calculation).unwrap();
    println!("{in_calculation} = {result}");
}
