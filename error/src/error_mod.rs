pub mod error {
    use std::{
        char,
        fmt::Error,
        fs::{self, File},
        io::{self, ErrorKind, Read},
    };

    // 通过panic!处理不可恢复错误
    pub fn panic_demo() {
        // panic!("crash and burn.");

        // 触发别的库的panic!
        let v = vec![10, 16, 96, 36, 78];
        // v[10];
    }

    // 使用Result枚举处理可恢复错误
    pub fn result_demo() {
        let greeting_file_result = File::open("hello.txt");

        // 通过枚举来处理可能出现的错误
        // 根据不同情况来提示信息
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("problem creating the file: {e:?}"),
                },
                _ => panic!("problem creating the file: {error:?}"),
            },
        };

        // 通过unwarp方法处理错误信息
        // let test_file = File::open("test.txt").unwrap();

        // 通过expect方法自定义错误信息
        // let test_file2 = File::open("test.txt").expect("test.txt file not found!");

        let result = read_username_from_file();

        match result {
            Ok(username) => println!("username = {username}"),
            Err(e) => println!("{e}"),
        };
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        // // 打开文件，返回result<T,V>
        // let username_file_result = File::open("hello.txt");

        // // 匹配错误与文件名
        // let mut username_file = match username_file_result {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };

        // let mut username = String::new();

        // // 返回用户名或错误
        // match username_file.read_to_string(&mut username) {
        //     Ok(_) => Ok(username),
        //     Err(e) => Err(e),
        // }

        // // 传播错误的快捷方式❓
        // let mut username_file = File::open("test.txt")?;
        // let mut username = String::new();
        // username_file.read_to_string(&mut username)?;
        // Ok(username)

        // // 更进一步简化代码
        // let mut username = String::new();

        // File::open("text.txt")?.read_to_string(&mut username)?;
        // Ok(username)

        // 更简短的写法
        fs::read_to_string("text.txt")

        /*
         * 只有当返回值为Result<T, V>或Option<T>或实现了FromResidual类型的函数时才能使用❓操作符
         * */
    }

    fn last_char_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}
