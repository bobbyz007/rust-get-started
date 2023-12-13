// 使用mod关键字将info声明为子模块
mod info;
mod teacher;

// 需要在主模块中声明student这个子模块，从而才能构建出模块树，不然编译运行不通过
mod student;

use modules::eat_at_reataurant;

fn main() {
    println!("Hello, world!");
    info::print_school_info(); // 这里我们使用::语法使用info下的print_school_info函数
    teacher::teaching::training(); // 这里的调用意思记作teacher目录下的teaching文件下的training函数

    eat_at_reataurant();
}
