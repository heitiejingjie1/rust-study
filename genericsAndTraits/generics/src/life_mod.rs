pub mod life {
    use std::net::ToSocketAddrs;

    pub fn dangling_references() {
        // 悬垂引用
        // let r;                   //----------------+ 'a  r 生命周期
        //                          //                |
        // {                        //                |
        //     let x = 10;          // -----+ 'b      | x 生命周期
        //     r = &x;              //      |         |
        // }                        //------|         |
        //                          //                |
        // println!("r: {r}");      //----------------+
    }

    pub fn test_life() {
        let str1 = "abcde".to_string();
        let str2 = "xyz".to_string();

        println!(
            "The largest str is {}",
            longest(str1.as_str(), str2.as_str())
        );

        longest2(str1.as_str(), str2.as_str());

        let novel = "Call me ishmeal. some years age....".to_string();
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        // 静态生命周期
        // 存活于整个程序
        let str3: &'static str = "静态生命周期";

        println!("{}", str3);
    }

    // 泛型生命周期
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    // 生命周期与参数有关
    fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // fn longest3<'a>(x: &str, y: &str) -> &'a str {
    //     let result = "hello world".to_string();
    //     result.as_str()
    // }

    // 结构体定义生命周期注解
    pub struct ImportantExcerpt<'a> {
        part: &'a str,
    }
}
