pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn move_right(self, distance: f64) -> Point2D {
        Point2D {
            x: self.x + distance,
            y: self.y,
        }
    }
}


// this method compiled intoo
// pub fn do_stuff(input: f64) -> f64 {
//     let p1 = Point2D { x: 3.0, y: 7.0 };
//     let p2 = Point2D {
//         x: p1.x + distance,
//         y: p1.y,
//     };
//     p2.x
// }
//
// then
//
// pub fn do_stuff(input: f64) -> f64 {
//     let p1_x: f64 = 3.0;
//     let p1_y: f64 = 7.0;
//     let p2_x: f64 = p1_x + input;
//     let p2_y: f64 = p1_y;
//     p2_x
// }
//
// then
//
// pub fn do_stuff(input: f64) -> f64 {
//     let p1_x: f64 = 3.0;
//     let p2_x: f64 = p1_x + input;
//     p2_x
// }
//
// and finaly simplified to
// pub fn do_stuff(input: f64) -> f64 {
//     3.0 + input
// }
pub fn do_stuff(input: f64) -> f64 {
    let p1 = Point2D { x: 3.0, y: 7.0 };
    let p2 = p1.move_right(input);
    p2.x
}