fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("xxx.com"),
        username: String::from("userrname"),
        ..user1
    };

    println!("user1: {}", user1.email);
    println!("user2: {}", user2.sign_in_count);

    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);
    println!("color: {:?}", black);
    println!("color: {:?}", point);

    let rect = Rectangle {
        width: 30,
        height: 20,
    };
    // println!("rect is : {:?}", rect);
    // println!("rect is : {:#?}", rect);
    println!("The area is {}", area(&rect));
    println!("The area is {}", rect.area());

    dbg!(&rect);

    let rect = Rectangle::squre(32);
    println!("squqre: {}", rect.area());
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn squre(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

// tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);