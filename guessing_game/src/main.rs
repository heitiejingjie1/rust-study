use std::io;

fn main() {
    println!("猜数字游戏!");
    println!("请输入一个猜测数字.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("输入错误");

    println!("你输入的数字为：{guess}");
}
