use crate::enums::enums;
use crate::patterns::patterns;
use crate::structs::structs;
use crate::type_conversion::{as_op, from_into, implicit_type_conversion_string, manual_type_conversion};

mod structs;
mod patterns;
mod type_conversion;
mod enums;

fn main() {
    println!("-----------------------------structs----------------------------------");
    structs();

    println!("------------------------------enums-----------------------------------");
    enums();

    println!("---------------------------match patterns-----------------------------");
    patterns();

    println!("---------------------------type conversion-----------------------------");
    implicit_type_conversion_string();
    // 以下都是显式类型转换
    manual_type_conversion();
    as_op();
    from_into();
}
