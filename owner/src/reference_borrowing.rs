pub mod reference {
    use std::usize;

    pub fn display() {
        reference_demo();
    }

    fn reference_demo() {
        // 引用与函数
        {
            let mut s = String::from("hello");
            let len = get_str_size(&mut s);
            println!("{s} length of {len}");
        }

        // 可变引用
        {
            let mut s = String::from("hello");

            let rms1 = &mut s;
            // let rms2 = &mut s; // 同一作用域，不可有两次可变引用
            // let rs1 = &s;   // 不可在可变引用情况下，声明不可变引用
            // println!("{rms1}, {rms2}");
            println!("{rms1}");
        }
    }

    // 传递引用就不会将参数的所有权传递到函数内
    // 还可再次使用该参数
    // 可变引用
    fn get_str_size(some_string: &mut String) -> usize {
        some_string.push_str(", world");
        some_string.len()
    }

    // 悬垂引用
    // fn dangling_demo() -> &String {
    fn dangling_demo() -> String {
        let s = String::from("hello");

        // &s
        s
    }
}
