use std::ops::Add;

// trait作为接口
pub fn trait_interface() {
    println!("struct add: {:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });

    // trait inheritance
    let my_paginate = MyPaginate { page: 1 };
    my_paginate.set_page(1);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(12);
}

#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

/// 一致性： trait接口或 trait实现至少一个在当前crate中定义
impl Add for Point {
    // associated type
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// trait inheritance
trait Page {
    fn set_page(&self, p: i32) {
        println!("page default: {:?}", p)
    }
}
trait PerPage {
    fn set_perpage(&self, num: i32) {
        println!("perpage default: {:?}", num)
    }
}

trait Paginate :Page + PerPage {
    fn set_skip_page(&self, num: i32) {
        println!("skip page: {:?}", num)
    }
}

#[allow(dead_code)]
struct MyPaginate {
    pub page: i32
}
// rustc complains if below 2 statements commented
impl Page for MyPaginate {}
impl PerPage for MyPaginate {}

// must also impl all its ancestor trait, 不影响之前的代码添加新功能
impl <T: Page + PerPage>Paginate for T {
}




