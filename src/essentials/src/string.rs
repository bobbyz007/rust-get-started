use std::num::ParseIntError;
use std::str;
use std::str::FromStr;

pub fn str_code() {
    let tao = str::from_utf8(&[0xE9, 0x81, 0x93]).unwrap();
    assert_eq!("道", tao);
    assert_eq!("道", String::from("\u{9053}")); // unicode code point

    let tao = '道';
    println!("U+{:x}", tao as u32); // 打印十六进制
    println!("{}", tao.escape_unicode());
    assert_eq!(char::from_u32(0x9053), Some('道'));
}

pub fn str_string() {
    let mut a = String::from("foo道");
    println!("{:p}", a.as_ptr());  // 堆中字节序列的指针地址
    println!("{:p}", &a);  // 在栈上指针的地址

    assert_eq!(a.len(), 6);  // 字节数

    a.reserve(10);
    assert_eq!(a.capacity() >= 15, true);

    let mut chars = a.chars();
    assert_eq!(Some('f'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('道'), chars.next());
}

pub fn str_to_other_type() {
    // rust已经为一些基本的原生类型实现了 FromStr trait
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);

    // Point实现了FromStr
    let p: Point = "{1,2}".parse().unwrap();  // 定义p需要指定类型Point
    assert_eq!(p, Point { x: 1, y: 2 });

    let result: Result<Point, ParseIntError> = "{1, u}".parse();
    match result {
        Ok(p) => println!("{:?}", p),
        Err(e) => println!("Error: {:?}", e)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.trim_matches(|p| p == '{' || p == '}').split(",").collect::<Vec<&str>>();
        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;
        Ok(Point { x, y })
    }
}