use std::{
    fmt::{Debug, Display},
    path::Components,
};

fn main() {
    traits_demo();
}

fn traits_demo() {
    // 可以在 Rust 裡定義一些行為，然後將這些行為套用到不同的類別上
    let kitty = Cat {
        name: String::from("Kitty"),
        age: 3,
    };

    kitty.fly();

    let wawa = Dog {
        name: String::from("Wawa"),
    };

    wawa.fly(); // 使用默认实现
    //
    bungee(&kitty);

    let post = Post {
        title: "first".to_string(),
        author: "heitiejingjie".to_string(),
        content: "我就是我".to_string(),
    };

    let weibo = Weibo {
        username: "fengmuxia".to_string(),
        content: "aiaiaiaiia".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    notify(&post);
    notify(&weibo);
    // notify4(&post);
    //
    type_transform();
}

// 定義一個 Trait
trait Flyable {
    fn fly(&self) {
        // Trait 可以有预设实现
        // 如果没有提供具体实现，则会使用默认实现
        print!("Default fly behavior");
    } // 只需定义函数签名就行了
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct Dog {
    name: String,
}
impl Flyable for Dog {}

// 挂载 Trait 到 Cat
impl Flyable for Cat {
    fn fly(&self) {
        println!("{} is flying!", self.name);
    }
}

// traits 实现多太
fn bungee(someone: &dyn Flyable) {
    // 只要 someone 实现了 Flyable trait，就可以调用 fly 方法
    someone.fly();
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// 使用特征作为函数参数
// 实现了特征的参数，都可以使用该函数
fn notify(item: &impl Summary) {
    println!("Breaking news. {}", item.summarize());
}

// 特征约束
// 与特征参数一致,特征参数只是特征约束的语法糖
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news. {}", item.summarize());
}

// 多重特征约束
// fn notify3<T: Summary + Display>(item: &T){}
fn notify4(item: &(impl Summary + Display)) {
    println!("{}", item);
}

// where 约束
fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// 函数返回中的impl Trait
// 只能返回一个特定的类型,不能返回不同的类型
// fn returns_summarizable () -> impl Summary {
//     Weibo {
//         username: "yinhao".to_string(),
//         content: "我要吃午饭".to_string()
//     }
// }

// 通过derive派生特征
// 被derive标记的对象会自动实现对应的默认特征代码，继承相应的功能

// tryinto:类型转换
fn type_transform() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("ten is less than one hundred.");
    }
}

// 特征对象
// fn returns_summarizable2 (switch: bool) -> impl Summary {
//     if switch{
//         Post {
//             title: "wojiushiwo".to_string(),
//             author: "heitiejingjie2".to_string(),
//             content: "哈哈哈哈哈".to_string()
//         }
//     } else {
//         Weibo {
//             username: "hahahha".to_string(),
//             content: "cbkshdn".to_string()
//         }
//     }
// }

trait Draw {
    fn draw(&self) -> String;
}

struct Button {
    weight: u32,
    height: u32,
    labal: String,
}

struct SelectBox {
    weight: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {}
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
