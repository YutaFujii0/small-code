// This cannot be compiled
// fn wont_compiled() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

pub fn compiled() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

pub fn compiled2() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
