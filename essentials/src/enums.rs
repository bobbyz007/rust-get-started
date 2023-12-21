pub fn enums() {
    let ip1 = IpAddr::V4(127, 1, 1, 1);
    let ip2 = IpAddr::V6("2001:0DB8:0000:0000:0008:0800:200C:417A".to_string());
    println!("ipv4: {:?}", ip1);
    println!("ipv6: {:?}", ip2);

    let msg = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 32, y: 23 };
    msg.call();
    msg2.use1();
}

#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

// enum behavior
impl Message {
    fn call(&self) {
        println!("enum value: {:?}", self)
    }
    fn use1(&self) -> &str {
        match self {
            Message::Quit => "1",
            Message::Move{x, y } => {
                println!("x: {:?}, y: {:?}", x, y);
                "2"
            },
            Message::Write(..) => "3",
            Message::ChangeColor(..) => "4"
        }
    }
}

#[allow(dead_code)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsSate),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsSate {
    Alabama,
    Alaska
}