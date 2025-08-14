/// for using procedural macro

use proc_macros::{Builder, define_struct_by_name, HelloMacroName, log_func_info, make_function};

fn main() {
    function_like_proc_macro();
    attribute_proc_macro();
    derive_proc_macro();
    derive_proc_macro_builder();
}

// function proc macro
fn function_like_proc_macro() {
    // 定义struct
    define_struct_by_name!(TestStruct);
    let s = TestStruct {data: 13};
    println!("{:?}", s);

    // 生成函数
    make_function!(fn double(usize) -> usize);
    double(1); // 2
    double(2); // 4
    double(3); // 6
    println!();
}

// attr proc macro
fn attribute_proc_macro() {
    my_function();
    println!();
}
#[log_func_info]
fn my_function() {
    println!("Hello, world!");
}

// derive proc macro
fn derive_proc_macro() {
    Pancakes::hello_macro();
    println!();
}

// derive procedural macro
trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacroName)]
#[allow(dead_code)]
struct Pancakes {
    weight: u16,
}

// derive proc macro：生成builder
fn derive_proc_macro_builder() {
    let com = Command::builder()
        .executable(Some("123".to_string()))
        .args(vec![1,2,3])
        .current_dir("id".to_string())
        .build();
    println!("command: {:?}", com);
}
#[derive(Builder, Debug)]
struct Command{
    executable: Option<String>,
    args: Vec<i32>,
    current_dir: String,
}