pub mod func {
    use core::num;
    use std::{i32, usize};

    pub fn display() {
        test_demo();
        // diverging_function();
        const_fn_demo();
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

    fn diverging_function() {
        // 发散函数，返回类型为"!"的函数
        // 可以被转换为任意类型
        fn deverges() -> ! {
            panic!("The function never returns");
        }

        let x: i32 = deverges();
        let y: f64 = deverges();

        deverges();
    }

    fn const_fn_demo() {
        // const fn 函数编译期计算
        const fn cube(number: usize) -> usize {
            number * number * number
        }

        const DIM: usize = cube(2);
        const ARR: [i32; DIM] = [0; DIM];

        println!("{:?}", ARR);
    }
}
