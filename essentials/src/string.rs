use std::fmt::{Display, Formatter};
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

// 所有这些格式化规则，对 println! 和 write! 宏同样适用
pub fn str_other_type_to_str() {
    // 字符串
    let s = format!("{} Rust", "Hello");
    assert_eq!(s, "Hello Rust");
    assert_eq!(format!("{:5}", "HelloRust"), "HelloRust");
    assert_eq!(format!("{:5.3}", "HelloRust"), "Hel  ");
    assert_eq!(format!("{:10}", "HelloRust"), "HelloRust ");
    assert_eq!(format!("{:<12}", "HelloRust"), "HelloRust   "); // 左对齐
    assert_eq!(format!("{:>12}", "HelloRust"), "   HelloRust"); // 右对齐
    assert_eq!(format!("{:^12.5}", "HelloRust"), "   Hello    "); // 截取5个字符
    assert_eq!(format!("{:=^12.5}", "HelloRust"), "===Hello===="); // =填充

    // 整数
    assert_eq!(format!("{:+}", -1234), "-1234"); // + 表示正负符号
    assert_eq!(format!("{:+}", 1234), "+1234");
    assert_eq!(format!("{:#x}", 1234), "0x4d2"); // #x 十六进制
    assert_eq!(format!("{:x}", 1234), "4d2"); // #x 十六进制
    assert_eq!(format!("{:>#15x}", 1234), "          0x4d2"); // #x 右对齐
    assert_eq!(format!("{:#b}", 1234), "0b10011010010"); // #b 二进制
    assert_eq!(format!("{:b}", 1234), "10011010010"); // b 二进制
    assert_eq!(format!("{:^20b}", 1234), "    10011010010     "); // 居中

    // 浮点数
    assert_eq!(format!("{:.4}", 1234.5678), "1234.5678");
    assert_eq!(format!("{:.2}", 1234.5618), "1234.56");
    assert_eq!(format!("{:.2}", 1234.5678), "1234.57");  // 截取时会四舍五入
    assert_eq!(format!("{:<10.4}", 1234.5678), "1234.5678 ");
    assert_eq!(format!("{:^10.2}", 1234.5678), " 1234.57  ");
    assert_eq!(format!("{:0^12.2}", 1234.5678), "001234.57000"); // 0填充
    assert_eq!(format!("{:e}", 1234.5678), "1.2345678e3");

    // 自定义类型，实现Display trait
    let city = City{
        name: "Beijing",
        lat: 39.90469,
        lon: -116.40717
    };
    assert_eq!(format!("{}", city), "Beijing: 39.905N 116.407W");
    println!("Format: {}", city);
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}
impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}{} {:.3}{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}