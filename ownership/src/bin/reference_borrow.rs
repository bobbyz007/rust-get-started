/// 借用规则：
/// 1. 借用的生命周期不能长于 出借方（拥有所有权的对象）的生命周期
/// 2. 共享不可变， 可变不共享， 类似读写锁。 多个不可变引用 或 1个可变引用，不能同时拥有。

#[allow(unused_mut)]
fn main() {
    let mut i = 20;
    let mut o = 5;
    compute(&i, &mut o);
    // compute(&i, &mut i);  //error 不可变借用和可变借用不可能同时存在。
    println!("o: {:?}", o);
}

fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output = 2;
    }
}
