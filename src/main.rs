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

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
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

    let rect3: Rectangle = Rectangle::square(3);
    println!("{:#?}", rect3);
}
