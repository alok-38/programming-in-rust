struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {width, height}
    }
    // Method to calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let reactangle = Rectangle::new(10, 5);

    println!("Width: {}", reactangle.width);
    println!("Height: {}", reactangle.height);
    println!("Area: {}", reactangle.area());
}