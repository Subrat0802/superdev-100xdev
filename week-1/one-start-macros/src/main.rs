struct Rect {
    width: u32,
    height: u32,
}

struct Square {
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // fn shape() -> String {
    //     return String::from("Subrat")
    // }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}


fn main() {
    let rect = Rect {
        width: 20,
        height: 30,
    };

    let square = Square { side: 20 };

    println!("{}", rect.area());
    // println!("{}", Rect::shape());
    println!("{}", square.area());
}
