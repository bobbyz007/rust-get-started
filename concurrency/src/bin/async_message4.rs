use concurrency::channel::{channel_async, channel_sync, channel_workload_sample};
use utils::trpl;

fn main() {
    trpl::run(async {
        channel_async();
        channel_sync();
        channel_workload_sample();
    })
}