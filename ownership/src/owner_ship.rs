pub mod owner_ship {
    pub fn display() {
        owner_string();
        owner_trans();
        owner_move();
    }

    // string 类型
    fn owner_string() {
        let s = "hello"; // 字面值类型, 不可更改
        let mut s1 = String::from("hello"); // String 类型
        s1.push_str(", world!");

        {
            let inner_s = "jiushiwo";
        }
        // 变量离开作用域自动drop
        // println!("{inner_s}");

        println!("{}", s1); // 输出: hello, world!
    }

    // 所有权转移
    #[allow(unused_variables)]
    fn owner_trans() {
        /*
         * 浅拷贝，所有权会发生转移，对copy类型（如整数、布尔值、字符等）进行赋值时，原变量仍然有效。
         */
        let a = 12;
        let b = a; // 整数类型是 Copy 类型，a 的值被复制到 b 中

        println!("a: {}, b: {}", a, b); // 输出: a: 12, b: 12

        /* * 深拷贝，所有权不会发生转移，对非 Copy 类型（如 String、Vec 等）进行赋值时，原变量不再有效。
         * 通过克隆来实现深拷贝，克隆会创建一个新的实例，原变量仍然有效。
         */
        let s1 = String::from("hello");
        let s2 = s1; // s1 的所有权转移到 s2, s1 不再有效
        // println!("{}", s1); // 这行代码会报错，因为 s1 已经不再有效
        println!("{}", s2); // 输出: Hello

        let s3 = String::from("hello");
        let s4 = s3.clone(); // 克隆 s3 的值到 s4, s3 仍然有效
        println!("s3 =  {}, s4 =  {}", s3, s4); // 输出: s3: hello, s4: hello

        // 重新赋值时，会抛弃原来的值
        {
            let mut s = "hello".to_string();
            s = "就是我".to_string();
            println!("{}, world!", s);
        }

        /*
         * 实现了copy特征的类型
         * 1、所有整数类型
         * 2、所有浮点数类型
         * 3、字符类型
         * 4、布尔类型
         * 5、元组类型，当且仅当其包含的类型也都实现 Copy 的时候。
         * */

        let x: &str = "hello,world"; // 字符串切片类型
        let y = x; // 字符串切片是 Copy 类型，x 的值被复制到 y 中
        println!("x: {}, y: {}", x, y); // 输出: x: hello,world, y: hello,world
    }

    // 函数所有权转移
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
}
