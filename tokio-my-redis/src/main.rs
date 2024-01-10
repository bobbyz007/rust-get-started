use crate::bytes::{bytes, bytes_bufmut, cursor};

mod examples;
mod bytes;

fn main() {
    bytes();
    bytes_bufmut();
    cursor();
}