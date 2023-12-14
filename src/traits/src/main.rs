use crate::trait_as_abstract_type::{trait_abstract_types};
use crate::trait_as_bound::{trait_bound};
use crate::trait_as_interface::{trait_interface};
use crate::trait_as_label::send_sync_safe;

mod trait_as_interface;
mod trait_as_bound;

mod trait_as_abstract_type;
mod trait_as_label;

fn main() {
    println!("-----------------------------trait interface----------------------------------");
    trait_interface();

    println!("-----------------------------trait bound--------------------------------------");
    trait_bound();

    println!("-----------------------------trait abstract types-----------------------------");
    trait_abstract_types();

    println!("-----------------------------trait label--------------------------------------");
    send_sync_safe();
}
