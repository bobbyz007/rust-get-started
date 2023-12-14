use crate::align::{mem_size, mem_size_composite};
use crate::pointer::{smart_pointer, smart_pointer_destruct, smart_pointer_shadowing};

mod align;
mod pointer;

fn main() {
    println!("-----------------------------alignment----------------------------------");
    mem_size();
    mem_size_composite();

    println!("-----------------------------smart pointer----------------------------------");
    smart_pointer();
    smart_pointer_shadowing();
    smart_pointer_destruct();
}
