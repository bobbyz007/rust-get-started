fn main() {
    let ip1 = IpAddr::V4(127, 1, 1, 1);
    let ip2 = IpAddr::V6("2001:0DB8:0000:0000:0008:0800:200C:417A".to_string());
    println!("ipv4: {:?}", ip1);
    println!("ipv6: {:?}", ip2);

    let msg = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 32, y: 23 };
    msg.call();
    msg2.use1();

    test_pattern();
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

fn test_pattern() {
    let value = value_in_cents(Coin::Quarter(UsSate::Alaska));
    println!("quarter value: {}", value);

    let i = 5;
    match i {
        3 => println!("3"),
        5 => println!("5"),
        // oth => println!("other: {}", oth),
        _ => println!("reroll"),
    }

    // if let is a syntax sugar for match
    let config_max = Some(89u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to {}", max);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        },
    }
}