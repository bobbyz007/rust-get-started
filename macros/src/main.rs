mod macro_by_example;
mod proc_macro_usage;

use utils::println_format_marker;
use crate::macro_by_example::meta_variable;
use crate::proc_macro_usage::{attribute_proc_macro, derive_proc_macro, derive_proc_macro_builder, function_like_proc_macro};

fn main() {
    println_format_marker("declarative macro");
    meta_variable();

    println_format_marker("procedural macro");
    function_like_proc_macro();
    attribute_proc_macro();
    derive_proc_macro();
    derive_proc_macro_builder();

}
