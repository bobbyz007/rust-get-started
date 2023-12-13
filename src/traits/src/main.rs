use crate::trait_as_bound::sum;
use crate::trait_as_interface::{MyPaginate, Page, Paginate, PerPage, Point};

mod trait_as_interface;
mod trait_as_bound;

mod trait_as_abstract_type;

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
}
