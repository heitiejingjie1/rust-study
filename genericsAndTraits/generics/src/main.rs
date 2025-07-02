fn main() {
    println!("Hello, world!");

    print!("{}", add(1, 2));
}

// 使用泛型函数
fn add<T>(a: T, b: T ) -> T {
    a + b 
}
