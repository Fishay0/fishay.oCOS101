// Rust program to calculate the area of a
// triangle for a given base and height


use std::io;

fn main()

 let mut input1 = String::new();
 let mut input2 = String::new();

 println!("Enter base: ");
 io::stdin().read_line(&mut input1).expect("Not a valid string")
 let base:f32 = input1.trim().parse().expect("Not a valid number")
