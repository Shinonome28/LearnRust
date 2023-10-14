#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[allow(dead_code)]
pub fn main3() {
    let rec1 = Rectangle {
        width: 10,
        height: 60,
    };

    let rec2 = Rectangle {
        width: 10,
        height: 50,
    };

    let a = rec1.area();

    println!("{a} width is {} height is {}", rec1.width(), rec1.height());

    dbg!(&rec1);
    dbg!(&rec2);

    println!(
        "rec1 can hold rec2? {}",
        if rec1.can_hold(&rec2) { "ok" } else { "no" }
    );
}
