use std::ops::Add;

#[derive(Debug)]
pub struct Point {
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
pub trait Page {
    fn set_page(&self, p: i32) {
        println!("page default: {:?}", p)
    }
}
pub trait PerPage {
    fn set_perpage(&self, num: i32) {
        println!("perpage default: {:?}", num)
    }
}

pub trait Paginate :Page + PerPage {
    fn set_skip_page(&self, num: i32) {
        println!("skip page: {:?}", num)
    }
}

pub struct MyPaginate {
    pub page: i32
}
// rustc complains if below 2 statements commented
impl Page for MyPaginate {}
impl PerPage for MyPaginate {}

// must also impl all its ancestor trait, 不影响之前的代码添加新功能
impl <T: Page + PerPage>Paginate for T {
}




