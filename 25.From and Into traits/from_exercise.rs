#[derive(Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

impl From<(u8,u8,u8)> for Pixel {
    fn from(tup:(u8,u8,u8)) -> Self {
        Pixel{ r: tup.0, g: tup.1, b: tup.2 }
    }
}


fn main(){
    let t = (0,0,0);
    let p = Pixel::from(t);
    println!("{:?}",p);
}