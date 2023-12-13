use std::ops::Add;

/// trait bound: a and b must all impl Add trait
pub fn sum<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}





