// use rand::prelude::*;
use std::io;

fn main() {
    // let mut rng = rand::thread_rng();
    // let x: u8 = rng.gen();
    // println!("x: {}", x);
    let a = [1, 2, 3, 4, 5];

    println!("Please an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please type a number!");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}
