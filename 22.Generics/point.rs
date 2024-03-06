use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T
}

impl<T: Display> Point<T> {
    fn display_point(&self){
        println!("Point({},{})", self.x, self.y)
    }
}

fn main(){
    let p: Point<u32> = Point { x: 5, y: 10 };
    p.display_point();
}