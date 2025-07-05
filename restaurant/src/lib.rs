// 可使用任何层级的路径
use std::io::{self, Write};
// 使用嵌套路径引入包
use std::cmp::Ordering;
// 使用glob运算符，引入下一路径的所有公有项
use std::collections::*;

mod front_of_house;

mod customer {
    // 可使用as关键字提供别名
    use crate::front_of_house::hosting as h;

    pub fn eat_at_restaurant() {
        h::add_to_waitlist();
    }
}

// 引入路径
// 只能创建特定作用域的捷径
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 使用use关键字后
    hosting::add_to_waitlist();

    // 在夏天订购一个黑麦吐司
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变面包类型
    meal.toast = "Wheat".to_string();

    println!("I like {} toast please!", meal.toast);

    // 枚举
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

/*
* 结构体公有，但成员不公有
* 枚举公有，其成员一并公有
* */
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // 通过super关键字访问父模块
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonl_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),
                seasonl_fruit: "apples".to_string(),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
