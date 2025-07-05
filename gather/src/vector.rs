pub mod vec {
    use std::collections::{HashMap, hash_map};

    pub fn test_vec() {
        // 新建vector
        let mut v: Vec<i32> = Vec::new();
        let mut v2 = vec![3, 4, 5];
        println!("{:?}", v);

        // 更新vector
        v.push(3);
        v.push(6);
        v.push(9);

        // 读取vector的元素
        // 通过索引获取vector的值
        let six = &v[1];
        // 通过get方法获取元素
        let nine = v.get(2);
        match nine {
            Some(nine) => println!("The nine element is {}", nine),
            None => println!("There is no element."),
        }
        println!("six = {}, nine = {:?}", six, nine);

        v.push(12);
        // 运行报错，所有权转移至v
        // println!("The two element is {six}");

        // 不可变遍历元素
        for ele in &v {
            println!("{ele }");
        }

        // 可变遍历元素
        for ele in &mut v {
            *ele += 1;
        }
        println!("{:?}", v);

        // 利用枚举存储不同类型的值
        let row = vec![
            SpreadSheetCell::Int(10),
            SpreadSheetCell::Float(56.9),
            SpreadSheetCell::Text("测试enum vec".to_string()),
        ];
        println!("{:?}", row);
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    pub fn test_string() {
        // 新建字符串
        let mut str = String::new();
        str = "initial string".to_string();
        let mut str2 = String::from("Initial string2");

        // 更新字符串
        // 使用push_str增加字符串
        str2.push_str(" update");
        // 使用push增加字符
        str2.push('.');
        println!("{}", str2);

        // 使用"+"运算符拼接字符串
        let str3 = str + &str2; // str所有权转移到str3
        // println!("{str}");
        println!("{str3}");

        // 使用format!宏拼接字符串
        let str4 = format!("{str2}-{str3}-{str3}");
        println!("{str4}");

        // 不能通过索引获取字符
        // let h = str4[0];
        // String 是一个 Vec<u8> 的封装
        // 通过索引只能获取一个字节数据

        // 字符串slice
        let str_slice = &"你好".to_string()[0..3];
        println!("{str_slice}");

        // 遍历字符串
        let zh_str = "风慕夏".to_string();

        // 通过字符遍历字符串
        for c in zh_str.chars() {
            println!("{c}");
        }
        // 通过字节遍历字符串
        for b in zh_str.bytes() {
            println!("{b}");
        }
    }

    pub fn test_hashmap() {
        // 新建一个hashmap
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert("blue".to_string(), 10);
        scores.insert("yellow".to_string(), 50);

        // 访问hashmap中的值
        // 通过get方法访问值，返回一个Option<T>
        let team_name = "blue".to_string();
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("{score}");

        // 遍历hashmap
        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        // 更新hashmap
        // 覆盖值，忽略旧值，插入新值
        scores.insert("blue".to_string(), 20);
        println!("{:?}", scores);
        // 只有键不存在时，才插入新值
        scores.entry("blue".to_string()).or_insert(30);
        scores.entry("red".to_string()).or_insert(60);
        println!("{:?}", scores);
        // 根据旧值更新新值
        let text = "hello world wonderful world";
        for word in text.split_whitespace() {
            let count = scores.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
        println!("{scores:?}")
    }
}
