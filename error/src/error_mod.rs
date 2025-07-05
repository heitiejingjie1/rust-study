pub mod error {
    use std::{fmt::Error, fs::File, io::ErrorKind};

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
        let test_file = File::open("test.txt").unwrap();

        // 通过expect方法自定义错误信息
        let test_file2 = File::open("test.txt").expect("test.txt file not found!");
    }
}
