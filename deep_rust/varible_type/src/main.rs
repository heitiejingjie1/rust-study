mod basic_data_type;
mod complex_data_type;

use crate::basic_data_type::type_test::display;
use crate::complex_data_type::complex_data::print;

fn main() {
    // variable_declaration();
    // variable_shadowing();
    // type_reference();
    // type_alias();
    // static_variable();
    display();
    print();
}

// 变量声明
fn variable_declaration() {
    println!("Hello, world!");

    let variable: i32 = 8;
    println!("{}", variable);
    println!("{:o}", variable);
    println!("{:b}", variable);
    println!("{:x}", variable);

    // 变量默认不可变
    let x = 5;
    let mut mut_x = 15;
    // x = 6;
    mut_x = 25;
    println!("{}", mut_x);

    // 模式匹配
    let (mut a, mut b) = (1, 2);
    // let point { x: ref a, y: ref b } = p;

    // 被使用的变量必须初始化
    let init_a: i32;
    // println!("{}", init_a);
}

// 变量遮蔽
fn variable_shadowing() {
    let x = "hello";
    println!("{x}");

    let x = 5;
    println!("{x}");

    // 可将不可变变量遮蔽为可变变量，反之亦然
}

// 类型推导
fn type_reference() {
    // 类型推导
    {
        let x = 8i32;
        let mut vec = Vec::new();

        vec.push(x);
        println!("{:?}", vec);
    }

    // 类型只推导一部分也行
    {
        let player_score = [("jack", 20), ("jell", 18), ("john", 19)];

        let players: Vec<_> = player_score
            .iter()
            .map(|&(player, _score)| player)
            .collect();

        println!("{:?}", players);
    }
}

// 类型别名
fn type_alias() {
    type Age = u32;

    let age: Age = 29;
    println!("{}", age);

    // 也可用于泛型场景
    type Double<T> = (T, Vec<T>);
    let score: Double<f64> = (65.5, Vec::new());
}

// 静态变量
fn static_variable() {
    // 与let语句一样，static语句也是一个模式匹配,生命周期是整个程序,必须马上初始化
    static GLOBAL: i32 = 0;

    {
        static INNER: f32 = 3.14;
    }

    println!("{GLOBAL}");

    // 修改mut static时需要unsafe修饰
    static mut MUT_G2: i64 = 100;
    unsafe {
        MUT_G2 = 101;
        // println!("{}", MUT_G2);
    }
}
