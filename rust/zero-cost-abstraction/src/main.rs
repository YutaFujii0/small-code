#![allow(dead_code)]

mod example;
mod example2;

fn main() {
    // example2();
}

fn example() {
    example::do_stuff();
}

fn example2() {
    let x = 16;
    println!("Result: {}", example2::add(x));
}