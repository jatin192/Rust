use std::io;
use std::cmp::Ordering;   //comparison

use rand::{thread_rng,Rng};

//Guessing Game

fn main() 
{
    let mut rand_num = rand::thread_rng().gen_range(1..101);   //1 - 100
    println!("random number = {}",rand_num);

    let mut input_ = String::new();

    println!("input->");
    io::stdin().read_line(&mut input_)
        .expect("Failed to Read line");

    let mut guess_:i32 = input_.trim().parse().expect("invalit input type");  // string -> integer

    match guess_.cmp(&rand_num)
    {
        Ordering::Less    => println!("Too less"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal   => println!("Equal huraa")

    }

}
