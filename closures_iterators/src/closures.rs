// 闭包
// 一个存储在变量里的类似函数的结构
// 可以保存在变量中或作为参数传递给其他函数的匿名函数
//

use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn test_closures() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // 闭包语法
    let add_one_v1 = |x: u32| -> u32 { x + 1 }; // 完整语法
    let add_one_v2 = |x: u32| x + 1; // 简写

    let example_closures = |x| x;
    let s = example_closures("hello".to_string());
    let n = example_closures(5.to_string());

    let mut list = vec![1, 6, 9];
    println!("Before defining closure: {list:?}");

    // 定义不可变闭包
    let immutable_closure = || println!("From Closures {list:?}");

    println!("Before defining closure: {list:?}");
    immutable_closure();
    println!("After defining closure: {list:?}");

    // 定义可变闭包
    let mut mutable_closure = || list.push(8);
    mutable_closure();
    println!("After defining closure: {list:?}");

    // 将闭包传递到新线程运行
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
