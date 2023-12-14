use crate::trait_as_abstract_type::{can_fly, Duck, dynamic_dispatch, fly_static, Foo, Pig, static_dispatch};
use crate::trait_as_bound::sum;
use crate::trait_as_interface::{MyPaginate, Page, Paginate, PerPage, Point};
use crate::trait_as_label::send_sync_safe;

mod trait_as_interface;
mod trait_as_bound;

mod trait_as_abstract_type;
mod trait_as_label;

fn main() {
    println!("struct add: {:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });

    // trait inheritance
    let my_paginate = MyPaginate { page: 1 };
    my_paginate.set_page(1);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(12);

    // trait bound
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1i32, 2i32), 3);

    // trait object
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);

    // impl trait
    let pig = Pig;
    assert_eq!(fly_static(pig), false);
    let duck = Duck;
    assert_eq!(fly_static(duck), true);
    can_fly(Pig);
    can_fly(Duck);

    // label: sized,copy,clone, send, sync
    send_sync_safe();
}
