struct Rectangle {
    width: u32,
    height: u32,
}
fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
fn main() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 30,
    };
    println!(
        "
        The area of the rectangle is {}
        square pixels.
        ",
        area(&rect)
    );
}
