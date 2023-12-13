use std::slice::SplitMut;

enum IpAddrKind {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Messaage{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Messaage {
    fn call(&self) {
        println!("xxx")
    }
}
fn plus_one(data: Option<i32>) -> Option<i32> {
    match data {
        Some(i) => Some(i + 1),
        None => None
    }
}

fn main() {
    let ip1 = IpAddr::V4(127, 1, 1, 1);
    let ip2 = IpAddr::V4(183, 1, 1, 1);
    println!("{:?}", ip1);

    let msg = Messaage::Write(String::from("hhelo"));
    let x = 32;
    let y = 23;
    let msg2 = Messaage::Move { x, y };
    println!("{:?}", msg2);
    msg.call();

    test_option();

    test_pattern();

    test_if_let();
}

// if let is a syntax sugar for match
fn test_if_let() {
    let config_max = Some(89u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to {}", max);
    }

}

fn test_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let str = "another string";

    let absent_number: Option<i32> = None;

    let data = some_number.unwrap();

    println!("{}", data);
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsSate),
}

#[derive(Debug)]
enum UsSate {
    Alabama,
    Alaska
}

fn test_pattern() {
    let value = value_in_cents(Coin::Quarter(UsSate::Alaska));
    println!("value: {}", value);

    let i = plus_one(Some(5));

    let i = 5;
    match i {
        3 => println!("3"),
        5 => println!("5"),
        // oth => println!("other: {}", oth),
        _ => println!("reroll"),
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