use crate::student::study::bye; // use相当于导入操作了

pub fn training() {
    println!("In training");

    // 其中crate的意思是从根crate进行查找也就是main.rs所在的目录，其实就是以绝对路径的方式进行调用。
    crate::student::study::do_home_work(); // 在teaching这个子模块中调用study.rs子模块的do_home_work函数

    bye(); // 直接使用bye函数
}