use concurrency::async_await::*;
use utils::trpl;

fn main() {
    trpl::run(async {
        async_await();
        async_future();
        async_unpin();
        async_pin_stack();
        async_pin_heap();
    })
}