pub mod generic {

    pub fn test() {
        let number_list = vec![23, 69, 15, 45, 36, 78];
        // let result = largest_i32(&number_list);
        // println!("The largest number is {result}");

        let char_list = vec!['j', 'f', 'm', 'b', 'z'];
        // let result = largest_char(&char_list);
        // println!("The largest char is {result}");

        println!("The largest value is{}", largest_T(&number_list));
        println!("The largest value is{}", largest_T(&char_list));

        let value_i32 = Point { x: 10, y: 20 };
        let value_f32 = Point { x: 11.1, y: 21.1 };
        let value_if32 = Point { x: 10, y: 11.1 };
        println!("{:?} {:?} {:?}", value_i32, value_f32, value_if32);
        println!("value_if32.x = {}", value_if32.x());
        println!("value_if32.y = {}", value_if32.y());

        let value_2 = Point2 { x: 5.0, y: 8.1 };
    }

    pub fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    pub fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // 泛型参数类型
    pub fn largest_T<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // 泛型结构体
    // 使用多个泛型参数
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }
    #[derive(Debug)]
    struct Point2<T> {
        x: T,
        y: T,
    }

    // 方法使用泛型
    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &U {
            &self.y
        }
    }

    // 使用方法具体类型
    impl Point2<f32> {
        fn distance_from_origin(&self) -> f32 {
            (&self.x.powi(2) + &self.y.powi(2)).sqrt()
        }
    }
}
