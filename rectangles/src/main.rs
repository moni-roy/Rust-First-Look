struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Area = {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
