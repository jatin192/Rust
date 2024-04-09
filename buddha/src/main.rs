// use std::library_name
use std::io;
use rand;
fn main() 
{
    let rand_num = rand::random::<f64>();  // Generating random Number
    println!("{}",rand_num);


    let mut str = String::new();          // empty string
    println!("str =");
    io::stdin().read_line(&mut str);    // user input
    println!("{}",str);
}
