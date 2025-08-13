mod owner_ship;

use crate::owner_ship::owner_ship::display;

fn main() {
    display();
}

fn dangling_reference() {
    /*
     * 悬垂引用:引用指向的值在引用存在期间被销毁
     * */
    let r; // 声明一个引用变量 r
    {
        let x = 42; // 在这个作用域内声明一个变量 x
        r = &x; // r 指向 x 的引用
        // x 在这里是有效的
    } // 作用域结束，x 被销毁

    // println!("r: {}", r); // 这行代码会报错，因为 r 指向的 x 已经被销毁
}

fn ownership_borrowinv() {
    /*
     * 所有权与借用:获取变量的引用
     * */

    let x = 5;
    let y = &x; // y 是 x 的引用，借用 x 的所有权

    assert_eq!(x, 5);
    assert_eq!(*y, 5); // 使用 * 解引用 y，获取 x 的值
    //
    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1); // 传递 s1 的引用，借用所有权
    println!("The length of '{}' is {}.", s1, len); // 输出: The length of 'hello' is 5.

    //同一作用域内，只允许一个可变引用和多个不可变引用
    {
        // 可变引用和不可变引用不能同时存在
        let s2 = &mut s1; // s2 是 s1 的可变引用
        // let s3 = &s1; // s3 是 s1 的不可变引用
        // let s4 = &s1; // s4 也是 s1 的不可变引用
        println!("s2: {}", s2); // 输出: s2: hello, s3: hello
    }
}

fn calculate_length(s: &mut String) -> usize {
    // 不可变引用
    // mut s 是一个可变引用，允许修改 s 的内容
    // 函数参数是 String 的引用，借用所有权
    s.push_str(", world!"); // 这行代码会报错，因为 s 是不可变引用，不能修改
    s.len() // 返回字符串的长度
}

#[allow(unused_variables)]
fn owner_move() {
    // 测试函数所有权转移
    let s = String::from("Hello, Rust!");
    function_owner_ship(s);
    // println!("s = {}", s); // 这行代码会报错，因为 s 的所有权已经转移到函数内部

    let x = 42;
    function_owner_copy(x);
    println!("x = {}", x); // 输出: x = 42, 因为整数类型是 Copy 类型，x 的值被复制到函数内部
}

// 所有权转移
#[allow(unused_variables)]
fn ownertrans() {
    /*
     * 浅拷贝，所有权会发生转移，对copy类型（如整数、布尔值、字符等）进行赋值时，原变量仍然有效。
     */
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权转移到 s2, s1 不再有效
    // println!("{}", s1); // 这行代码会报错，因为 s1 已经不再有效
    println!("{}", s2); // 输出: Hello

    let x: &str = "hello,world"; // 字符串切片类型
    let y = x; // 字符串切片是 Copy 类型，x 的值被复制到 y 中
    println!("x: {}, y: {}", x, y); // 输出: x: hello,world, y: hello,world

    /* * 深拷贝，所有权不会发生转移，对非 Copy 类型（如 String、Vec 等）进行赋值时，原变量不再有效。
     * 通过克隆来实现深拷贝，克隆会创建一个新的实例，原变量仍然有效。
     */
    let s3 = String::from("hello");
    let s4 = s3.clone(); // 克隆 s3 的值到 s4, s3 仍然有效
    println!("s3: {}, s4: {}", s3, s4); // 输出: s3: hello, s4: hello
}

#[allow(unused_variables)]
fn function_owner_ship(some_string: String) -> String {
    // 函数参数是 String 类型，所有权转移到函数内部
    println!("Received string: {}", some_string);
    // some_string 在函数结束时被销毁
    some_string // 返回值，所有权转移回调用者
}

#[allow(unused_variables)]
fn function_owner_copy(some_integer: i32) {
    // 函数参数是 i32 类型，整数类型是 Copy 类型，值被复制到函数内部
    println!("Received integer: {}", some_integer);
    // some_integer 在函数结束时被销毁
}
