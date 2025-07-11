use std::{
    env::{self},
    process,
};

pub mod minigrep;
use minigrep::Config;

fn main() {
    // // 读取命令行参数值
    // let args: Vec<String> = env::args().collect();

    // // 将参数值存进变量
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // 高效率改进
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // 读取文件并搜索字符
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
