use concurrency::thread::*;

fn main() {
    thread_create();
    thread_create_builder();
    thread_local();
    thread_park();
    thread_send();
    thread_send_sync();
}