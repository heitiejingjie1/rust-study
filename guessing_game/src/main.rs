use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("猜数字游戏!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("随机数为:{secret_num}");

    loop {
        println!("请输入一个猜测数字.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("输入错误");
        // let guess: u32 = guess.trim().parse().expect("请输入一个纯数字。"); // 转换类型，trim()去除字符串标志，parse()尝试转换
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你输入的数字为：{guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("太小了。"),
            Ordering::Equal => {
                println!("恭喜你！猜对了。");
                break;
            }
            Ordering::Greater => println!("太大了。"),
        }
    }
}
