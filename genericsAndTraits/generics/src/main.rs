use std::array;

fn main() {
    println!("Hello, world!");

    println!("{}", add(1, 2));
    print_point();
    print_const();
}

// 使用泛型函数
fn add<T: std::ops::Add<Output = T>>(a: T, b: T ) -> T {
    a + b 
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    largest
}

// 结构体中使用泛型

struct Point<T> {
    x: T,
    y: T,
}

// 结构体中使用多个泛型
struct Point2<T, U> {
    x: T,
    y: U,
}

fn print_point () {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };

    let arr = [1, 2, 3, 4, 5];
    let largest_num = largest(&arr);
    println!("Largest number: {}", largest_num);

    // let p3 = Point{ x: 1, y : 2.0 }; // 这会报错，因为 x 和 y 的类型不一致
    let p3 = Point2 { x: 1, y: 2.0 };

    println!("Point 1: ({}, {})", p1.x, p1.y);

    p1.print_x();
    println!("point: {}", p2.distance_from_origin()); 
}

// 方法中使用泛型
impl <T: std::fmt::Debug> Point<T>  {

    fn print_x(&self)  -> &T{
        println!("x: {:?}", self.x);
        &self.x
    }
}

// 为具体的泛型类型定义方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// const 泛型
fn display_arr<T: std::fmt::Debug> (arr: &[T]) {
    println!("{:?}", arr);  
}

fn display_arr2<T: std::fmt::Debug, const N: usize> (arr: [T; N]) {
    println!("{:?}", arr);  
}


fn print_const() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    display_arr(&arr);
    display_arr(&arr2);

    display_arr2(arr);
    display_arr2(arr2);
}
