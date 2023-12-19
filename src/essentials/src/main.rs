use crate::closure::{closure_fn, closure_fnmut, closure_fnonce, closure_higher_ranked_lifetime, closure_move, closure_return, closure_vector};
use crate::enums::enums;
use crate::iterator::iterator_adapter;
use crate::patterns::{pattern_builder, pattern_visitor, patterns};
use crate::structs::structs;
use crate::type_conversion::{as_op, from_into, implicit_type_conversion_string, manual_type_conversion};

mod structs;
mod patterns;
mod type_conversion;
mod enums;
mod closure;
mod iterator;

fn main() {
    println!("-----------------------------structs----------------------------------");
    structs();

    println!("------------------------------enums-----------------------------------");
    enums();

    println!("---------------------------match patterns-----------------------------");
    patterns();
    pattern_builder();
    pattern_visitor();

    println!("---------------------------type conversion-----------------------------");
    implicit_type_conversion_string();
    // 以下都是显式类型转换
    manual_type_conversion();
    as_op();
    from_into();

    println!("-------------------------------closure--------------------------------");
    closure_fn();
    closure_fnmut();
    closure_fnonce();
    closure_move();
    let _ = closure_return(1);
    closure_vector();
    closure_higher_ranked_lifetime();
    iterator_adapter(); // 自定义迭代器适配器
}
