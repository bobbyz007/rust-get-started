use std::ops::Add;

/// trait bound: a and b must all impl Add trait
fn sum<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn sum2<T>(a: T, b: T) -> T
    where T: Add<T, Output = T>{
    a + b
}

pub fn trait_bound() {
    // trait bound
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1i32, 2i32), 3);

    assert_eq!(sum2(1, 2), 3);
    assert_eq!(sum2(1i32, 2i32), 3);
    println!("10 + 38 = {:?}", sum2(10, 38));
}


