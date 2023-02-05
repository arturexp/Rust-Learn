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
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle::square(50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can react1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can react1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect1 is {:#?}", rect1);

    println!("rect4 is square rectangle with size {}", rect4.width);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Ассоциированная функция - конструктор для возврата
    // нового экземляра структуры.
    // Вызываются ::
    // Возвращает квадрат
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
