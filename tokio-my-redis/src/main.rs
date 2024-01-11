use crate::bytes::{bytes, bytes_bufmut, cursor};
use crate::mini_tokio::mini_tokio;

mod examples;
mod bytes;
mod mini_tokio;

fn main() {
    bytes();
    bytes_bufmut();
    cursor();

    mini_tokio();
}