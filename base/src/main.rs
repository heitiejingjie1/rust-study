use num::complex::Complex; // 引入 num crate 中的 Complex 类型

const MAX_LENGTH: usize = 10; // 常量，类型为usize

fn main() {
    // base_type();
    // test_variable();
    // test_struct();
    println!("{}", MAX_LENGTH);
    // variable_shield();
    // test_float();
    // test_range();
    complex_test();
    statement();
    assert_eq!(statement(), ());
}

// 语句
fn statement() {
    // let b = let a = 10;
    let a = 1;
    let x = if a % 2 == 1 { "odd" } else { "even" };
}

// 测试复数
fn complex_test() {
    let a = Complex { re: -78, im: 123 };
    let b = Complex::new(11, 213); // 使用 num crate 的 Complex 类型
    let result = a + b; // 复数相加

    println!("a + b = {} + {}i", result.re, result.im);
}

// 序列
fn test_range() {
    for i in 1..=10 {
        println!("i = {}", i);
    }

    for i in 'a'..='z' {
        println!("i = {}", i);
    }
}

fn base_type() {
    let a = 10; // a 不可变 

    let b: i32 = 20; // 主动指定b的类型为i32

    let c = 30i32; // c 可变
    let d = 40_i32; // d 可变，类型为i32 
    //
    let e = add(add(a, b), add(c, d)); // e 可变，类型为_i32

    println!("(a + b) + (c + d) = {}", e);

    let _y = 100;
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

struct Struct {
    e: i32,
}

fn test_struct() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);

    [c, .., d, _] = [1, 2, 3, 4, 5];

    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

fn test_variable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a: {}, b: {}", a, b);

    b = true; // b 可变
    assert_eq!(a, b);
}

// 变量遮蔽
fn variable_shield() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1; // 遮蔽了之前的x
    println!("The value of x is: {}", x);
    {
        let x = x * 2; // 在这个作用域内，x被遮蔽
        println!("The value of x in inner scope is: {}", x);
    }
    println!("The value of x after inner scope is: {}", x); // 仍然是之前的x
}

// 浮点数
fn test_float() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("      0.1 + 0.2 = {}", (abc.0 + abc.1).to_bits());
    println!("            0.3 = {}", abc.2.to_bits());

    println!("xyz (f32)");
    println!("      0.1 + 0.2 = {}", (xyz.0 + xyz.1).to_bits());
    println!("            0.3 = {}", xyz.2.to_bits());

    assert!((abc.0 + abc.1) == abc.2, "f32 浮点数精度问题");
    assert!((xyz.0 + xyz.1) == xyz.2, "f64 浮点数精度问题");
}
