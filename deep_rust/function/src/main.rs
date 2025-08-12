mod function;

use crate::function::func::display;

fn main() {
    display();

    // 传递参数和返回状态码由单独的API来完成
    for arg in std::env::args() {
        println!("Arg: {arg}");
    }

    std::process::exit(0);
}
