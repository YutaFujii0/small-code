fn main() {
    let x = 16;
    println!("Result: {}", add(x));
}

fn add(x: i32) -> i32 {
    x + 2 - 3 * 5 - x
}