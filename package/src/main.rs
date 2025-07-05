use crate::garden::vegetables::Asparagus;

pub mod garden;

extern crate restaurant;

fn main() {
    println!("Hello, world!");
    let plant = Asparagus {};
}

// 一个包（package）可以包含多个二进制 crate 项和一个可选的库 crate。
// package: Cargo 的一个功能，它允许你构建、测试和分享 crate.
// -- 提供一系列功能的一个或者多个 crate 的捆绑
// -- 会包含一个Cargo.toml文件,阐述如何构建这些crate

// crates:一个模块的树形结构，它形成了库或可执行文件项目。
//  -- 二进制crate : 必须有一个名为 main 函数来定义, 可编译为可执行文件
//  -- 库crate: 并没有 main 函数，它们也不会编译为可执行程序,定义了可供多个项目复用的功能模块

// modules和use: 允许你控制作用域和路径的私有性。

// paths: modules的路径
