// 所有权
pub mod owner_ship {
    pub fn owner_display() {
        owner_string();
        owner_trans();
        owner_move();
        println!("========^======^^=");
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

// 引用与借用
// 将创建引用的行为称为借用(borrowing)
pub mod reference {
    use std::usize;

    pub fn reference_display() {
        reference_test();
    }

    fn reference_test() {
        let mut s = "就是我".to_string();
        println!("{s} len is {}", calculate_len(&s));
        change_str(&mut s);

        // 同一作用域只能创建一次可变引用
        // 但可创建多次不可变引用
        // 在创建可变引用之后,不能再次使用不可变引用,会造成数据竞争
        {
            let mut inner_str = "hello".to_string();

            let str1 = &inner_str;
            let str2 = &inner_str;

            println!("str1 = {str1}, str2 = {str2}");

            let mut_str1 = &mut inner_str;
            // let mut_str2 = &mut inner_str;
            mut_str1.push_str(", world!");
            // let after_str1 = &inner_str;

            println!("mut_str1 = {mut_str1}");
        }
    }

    // 参数引用传递
    // 允许函数使用值，但不获取所有权
    // 不能修改所引用的变量,没有改变量的所有权
    fn calculate_len(str: &String) -> usize {
        // str.push_str(", 怎么样");
        str.len()
    }

    //可变引用
    fn change_str(str: &mut String) {
        str.push_str(", 怎么样");
        println!("{str}");
    }

    // 悬垂引用
    fn dangling_reference() {
        let s = "hahh".to_string();
        // &s
    } // s的值在离开此作用域时被丢弃，无法再次借用
}

// 切片
// 允许你引用集合中一段连续的序列，而不用引用整个集合
// 也不拥有该集合所有权
pub mod slice {
    pub fn slice_display() {
        let str = "hello world jiudhiwo".to_string();
        let slice = first_word_slice(&str);

        // str.clear();
        println!("{slice}");
    }

    //引入问题
    fn first_word(str: &String) -> usize {
        let bytes = str.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        return str.len();
    }

    // 修改函数
    fn first_word_slice(str: &str) -> &str {
        let bytes = str.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &str[0..i];
            }
        }

        return &str[..];
    }

    // 字符串切片
    fn string_slice() {
        let str = "hello world".to_string();
        let hello = &str[..5];
        let world = &str[6..];

        println!("{hello}, {world}");
    }
}
