#![allow(dead_code)]

mod example;
mod example2;

fn main() {
    example2();
}

fn example() {
    example::do_stuff(2.0);
}

fn example2() {
    let x = 3;
    println!("Result: {}", example2::add(x));
    // println!("Result: {}", -13);
}
