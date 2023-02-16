extern crate libmath;

use std::io;

use crate::libmath::*;

fn main() -> io::Result<()> {
    
    let mut user_input1 = String::new();
    let mut user_input2 = String::new();

    println!("This code is very complex, adds 2 values");
    
    println!("Insert value 1:");

    let stdin = io::stdin(); // We get `Stdin` here.
    
    _ = stdin.read_line(&mut user_input1);
    let new_len1 = user_input1.trim_end().len();
    user_input1.truncate(new_len1);

    println!("Insert value 2:");
    _ = stdin.read_line(&mut user_input2);
    let new_len2 = user_input2.trim_end().len();
    user_input2.truncate(new_len2);
    
    let _num1: i32 = user_input1.parse().unwrap();
    let _num2: i32 = user_input2.parse().unwrap();
    
    let sum = add(_num1, _num2);
    
    println!("The result is {} ", sum);

    Ok(())
}