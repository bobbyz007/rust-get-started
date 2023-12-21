pub fn do_home_work() {
    println!("do home work");

    println!("below using crate keyword");

    crate::student::play::play_football(); // 使用crate关键字调用play子模块中play_football函数

    println!("below using super keyword");

    super::play::play_football(); // 使用super关键字调用play子模块中play_football函数

}

pub fn bye() {
    println!("goodbye!");
}