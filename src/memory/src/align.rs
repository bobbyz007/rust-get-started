use std::mem;

#[allow(dead_code)]
struct S {
    a: u8,
    b: u32,
    c: u16
}

pub fn mem_size() {
    // 实际7个字节，但对齐需要要占用8个字节， 而且结构体字段在内存布局可能需要重排序
    println!("mem size: {:?}", mem::size_of::<S>())
}

pub fn mem_size_composite() {
    println!();
    println!("Box<u64>: {:>}", mem::size_of::<Box<u64>>());
    println!("A: {:>}", mem::size_of::<A>());
    println!("B: {:>}", mem::size_of::<B>());
    println!("N: {:>}", mem::size_of::<N>());
    // enum大小考虑tag和自定义数据的大小
    println!("E: {:>}", mem::size_of::<E>());
    println!("U: {:>}", mem::size_of::<U>());
}

#[allow(dead_code)]
struct A {
    a: u32,
    b: Box<u64>
}
struct B(i32, f64, char);
struct N;
#[allow(dead_code)]
enum E {
    H(u32),
    M(Box<u32>)
}
#[allow(dead_code)]
union U {
    u: u32,
    v: u64
}
