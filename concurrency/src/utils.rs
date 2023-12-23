use std::time::SystemTime;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator};

// 迭代器模式
pub fn rayon_par_iter() {
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("rayon: {}", sum_of_squares(&v));
    let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    increment_all(&mut v);
    println!("rayon: {:?}", v);

}
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum()
}

fn increment_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(|p| *p += 1);
}

// rayon的性能对比
pub fn rayon_on_off(limit: i64) {
    let arr:Vec<i64> = (1..limit).collect();

    // 不使用rayon
    let time_a = SystemTime::now();
    let result = sum_of_add(&arr);
    println!("{}",result);

    // 基于rayon
    let time_b = SystemTime::now();
    let result = sum_of_add_with_rayon(&arr);
    println!("{}",result);
    let time_c = SystemTime::now();

    println!("duration with/without rayon: {:?} / {:?}", time_c.duration_since(time_b), time_b.duration_since(time_a));
}
fn sum_of_add(input: &[i64]) -> i64 {
    input.iter()
        .map(|&i| i + i)
        .sum()
}
fn sum_of_add_with_rayon(input: &[i64]) -> i64 {
    input.par_iter()
        .map(|&i| i + i)
        .sum()
}

// join
pub fn rayon_join(n: u32) {
    let time_a = SystemTime::now();
    println!("{:?}", fib(n));
    let time_b = SystemTime::now();
    println!("{:?}", fib_with_rayon(n));
    let time_c = SystemTime::now();
    println!("duration with/without rayon: {:?} / {:?}", time_c.duration_since(time_b), time_b.duration_since(time_a));
}

fn fib(n: u32) -> u32 {
    if n < 2 { return n; }
    fib(n - 1) + fib(n - 2)
}
fn fib_with_rayon(n: u32) -> u32 {
    if n < 2 { return n; }
    // 并发计算
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2));
    a + b
}