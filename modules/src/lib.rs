// use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to wait list");
        }

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn main() {
    eat_at_reataurant();
}

pub fn eat_at_reataurant() {
    hosting::add_to_waitlist();

    println!("{:?}", back_of_house::Appetizer::Soup);
}

mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer{
        Soup,
        Salad
    }
}