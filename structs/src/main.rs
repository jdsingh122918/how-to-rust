#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 32,
        height: 52,
    };

    let rect2 = Rectangle {
        width: 38,
        height: 13,
    };

    let square1 = Rectangle::square(3);

    println!("Area of struct rectangle is {}", rect1.area());
    println!("Rectangle is: {:#?}", rect1);
    println!(
        "Rectangle 1 can hold rectangle 2: {}",
        rect1.can_hold(&rect2)
    );
    println!("Area of the square is : {}", square1.area());
}
