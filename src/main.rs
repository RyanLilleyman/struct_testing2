#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width || self.height > other.height
    }
}
fn main() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 30,
    };
    let rect2: Rectangle = Rectangle { width: 40, ..rect };

    println!("{:#?}", &rect);
    println!(
        "
        The area of the rectangle is {}
        square pixels.
        ",
        rect.area()
    );

    println!("{:#?}", rect2);
    println!(
        "
        Can {:#?}
        hold {:#?}
        ? {:#?}",
        &rect,
        &rect2,
        rect.can_hold(&rect2)
    );
}
