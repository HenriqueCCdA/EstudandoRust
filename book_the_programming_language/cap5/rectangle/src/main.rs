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

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can react1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can react2 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(2);

    println!("{:?}", square);
}



/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}


fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area off the rectangle is {} squares pixels.", rect1.area());
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    }

    println!("{:?}", rect1);
}


fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} sqaure pixels", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


fn main() {
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} squares pixels.", area(rect1));

}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area off the rectangle is {} squares pixels.", area(&rect1));

    println!("{:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height    
}
*/
