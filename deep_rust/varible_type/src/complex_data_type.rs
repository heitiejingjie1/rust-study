pub mod complex_data {
    use std::mem;

    pub fn print() {
        tuple_demo();
        struct_demo();
        enum_demo();
    }

    fn tuple_demo() {
        // 元组
        // 元素没有名字
        let x = (1_i32, false);
        let y = (1_i16, (false, 3.14_f64));

        // 如果元组只有一个元素，需在元素后加一个，
        {
            let x = (10_i32,); // 这是元组 
            let y = (10); // 这是一个i32类型
        }

        // 访问元组
        // 1、模式匹配
        // 2、索引
        {
            let t = (29, 290);
            let (a, b) = t;

            let x = t.1;
            let y = t.0;

            println!("{a}, {b}, {x}, {y}");
        }

        // 单元元组
        // 占用0字节空间
        {
            let x = ();
            println!("{}", mem::size_of_val(&x));
        }
    }

    fn struct_demo() {
        // 结构体类型
        // 与元组类似，但元素有名字,不能使用自动推导类型，必须显式指定类型
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        // 初始化struct
        let p = Point { x: 2, y: 6 };
        println!("{:?}", p);

        // 简略写法
        let (x, y) = (3, 7);
        let p1 = Point { x, y };

        // 访问元素
        // 模式匹配
        let Point { x: px, y: py } = p1;
        // .操作符
        let pxp = p1.x;
        let pyp = p1.y;
        println!("{px} {py} {pxp} {pyp}");

        // 空结构体
        struct Str_Point {}
    }

    fn tuple_struct_demo() {
        // 结构体元组
        // 类型有名字，元素没名字
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        // 让我们非常方便地在一个类型的基础上创建了一个新的类型
        {
            struct Test(i32);

            fn f1(value: Test) {}
            fn f2(value: i32) {}

            let v: i32 = 12;
            // f1(v);
            f2(v);
        }
    }

    fn enum_demo() {
        // 枚举类型 enum
        // 成员与struct类型一致
        #[derive(Debug)]
        enum Number {
            Int(i32),
            Float(f32),
        }

        // 使用enum需要用到"模式匹配"
        fn read_number(num: &Number) {
            match num {
                // 如果匹配到了Number::Int，value就是i32
                &Number::Int(value) => println!("Integer {value}"),
                &Number::Float(value) => println!("Float {value}"),
            }
        }

        let n = Number::Int(29);
        read_number(&n);

        println!("size of number: {}", mem::size_of::<Number>());
        println!("size of i32: {}", mem::size_of::<i32>());
        println!("size of f32: {}", mem::size_of::<f32>());
    }
}
