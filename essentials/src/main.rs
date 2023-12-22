use crate::closure::{closure_fn, closure_fnmut, closure_fnonce, closure_higher_ranked_lifetime, closure_move, closure_return, closure_vector};
use crate::enums::enums;
use crate::iterator::iterator_adapter;
use crate::patterns::{pattern_builder, pattern_visitor, patterns};
use crate::string::{str_code, str_other_type_to_str, str_string, str_to_other_type};
use crate::structs::structs;
use crate::type_conversion::{as_op, auto_deref, from_into, implicit_type_conversion_string, manual_type_conversion};
use utils::println_format_marker;
use crate::collection::cmp_order;
use crate::error::{err_handled_application, err_handled_library};
use crate::regex::{regex_group_name, regex_match};

mod structs;
mod patterns;
mod type_conversion;
mod enums;
mod closure;
mod iterator;
mod string;
mod collection;
mod error;
mod regex;

fn main() {
    println_format_marker("structs");
    structs();

    println_format_marker("enums");
    enums();

    println_format_marker("match patterns");
    patterns();
    pattern_builder();
    pattern_visitor();

    println_format_marker("type conversion");
    implicit_type_conversion_string();
    // 以下都是显式类型转换
    manual_type_conversion();
    as_op();
    from_into();
    auto_deref();

    println_format_marker("closure");
    closure_fn();
    closure_fnmut();
    closure_fnonce();
    closure_move();
    let _ = closure_return(1);
    closure_vector();
    closure_higher_ranked_lifetime();
    iterator_adapter(); // 自定义迭代器适配器

    println_format_marker("string");
    str_code();
    str_string();
    str_to_other_type();
    str_other_type_to_str();

    println_format_marker("collection");
    cmp_order();

    println_format_marker("error");
    err_handled_application();
    err_handled_library();

    println_format_marker("regex");
    regex_match();
    regex_group_name();

}
