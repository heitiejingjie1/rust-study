pub mod func {
    use std::i32;

    pub fn display() {
        test_demo();
    }

    fn add1(x: (i32, i32)) -> i32 {
        x.0 + x.1
    }

    fn add2((x, y): (i32, i32)) -> i32 {
        return x + y;
    }

    fn test_demo() {
        println!("{}", add1((3, 5)));

        // 函数可以当成头等公民被复制到一个值中，这个值可以当函数一样使用
        let func_t = add2;
        println!("{}", func_t((10, 19)));

        // 可以将函数类型转换为fn类型
        // let mut func_1 = add1; // 将该值指向add1
        let mut func_1 = add1 as fn((i32, i32)) -> i32;
        func_1 = add2; // 再将该值指向add2
    }

    fn diverging_function() {}
}
