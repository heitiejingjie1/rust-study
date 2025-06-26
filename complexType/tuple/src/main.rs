fn main() {
    tuple_demo();
}

fn tuple_demo() {
    // 元组中的元素长度固定，顺序也固定
    let tuple: (i32, f64, String) = (500, 6.4, "Rust".to_string());

    // 模式匹配解构元组
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 直接访问元组元素
    let first = tuple.0;
    let second = tuple.1;

    println!("{} {} ", first, second);
}
