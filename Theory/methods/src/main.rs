#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn new_square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("Can rect1 hold rect2 ? {}", rect2.can_hold(&rect1));

    println!(
        "The area of the rectangle is {} square pixels.", rect1.calculate_area()
    );

    let sq = Rectangle::new_square(5);

    println!("New square is {:?}", sq)
}