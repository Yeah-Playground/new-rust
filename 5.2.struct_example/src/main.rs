#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("The area of rect is  {} pixels", area(&rect1));
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let _r = Rectangle::square(10);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
