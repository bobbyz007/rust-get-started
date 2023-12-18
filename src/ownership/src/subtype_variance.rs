// 参考： Why could this work about lifetime?
// https://users.rust-lang.org/t/why-could-this-work-about-lifetime/104015
pub fn lifetime_covariant() {
    let t0 = 3;
    let t = &t0;
    {
        let s0 = 2;
        let s = &s0;
        // 为什么允许调用，是因为'a 的covariance，长生命周期参数可以coerce into 短生命周期参数
        // 因为在rust 长生命周期其实是短生命周期的 sub type
        // 参考： Subtyping and Variance： https://zhuanlan.zhihu.com/p/525175176
        foo(s, t);
    }
    println!("{}", t);
}

#[allow(unused_variables, unused_mut, dead_code)]
pub fn lifetime_invariant() {
    let t0 = 3;
    let mut t = &t0;
    {
        let s0 = 2;
        let mut s = &s0;
        // 在rust 长生命周期其实是短生命周期的 sub type
        // 参考： Subtyping and Variance： https://zhuanlan.zhihu.com/p/525175176

        // ERR!  因为基于mut 此时s是 invariant，也就是生命周期参数不能转换，所以会报错
        // foo_complain(&mut s, &mut t);
    }
    println!("{}", t);
}

// lifetime 'a outlive 'b
fn foo<'a, 'b>(s: &'a i32, t: &'b i32) where 'a: 'b {
    println!("{} {}", s, t);
}

#[allow(dead_code)]
fn foo_complain<'a, 'b>(s: &mut &'a i32, t: &mut &'b i32) where 'a: 'b {
    println!("{} {}", s, t);
}
