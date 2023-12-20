/// NLL: Non Lexical Lifetime 基于非词法作用域生命周期的 借用检查
pub fn nll_borrow() {
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    capitalized(slice);

    // slice的可变借用基于词法作用域是到函数结尾的， 但NLL可以优化，可变借用到此就结束了， 后续可合法使用data
    data.push('d');
}

#[allow(unused_variables)]
fn capitalized(data: &mut [char]) {
    // do something
}