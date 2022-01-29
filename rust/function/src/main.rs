mod fn_as_argument;
mod return_closure;

use crate::fn_as_argument::*;
use crate::return_closure::*;

fn main() {
    let answer = do_twice(add_one, 3);
    println!("function as argument: 3 add twice {}", answer);

    let nums: Vec<i32> = (0i32..10).map(compiled()).collect();
    println!("closure returned using impl statement {:?}", nums);

    let nums2: Vec<i32> = (0i32..10).map(compiled2()).collect();
    println!("closure returned using Box type {:?}", nums2);
}
