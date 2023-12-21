mod thread;

use utils::println_format_marker;
use crate::thread::{thread_create, thread_create_builder, thread_local, thread_park, thread_send, thread_send_sync};

fn main() {
    println_format_marker("thread");
    thread_create();
    thread_create_builder();
    thread_local();
    thread_park();
    thread_send();
    thread_send_sync();


}
