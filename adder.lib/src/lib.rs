use std::{fmt::format, io::SeekFrom};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("hello {name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn not_works() {
        let result = add(2, 3);
        assert_ne!(!result, 5);
    }

    // 利用result处理测试
    #[test]
    fn it_works2() -> Result<(), String> {
        let result = add(6, 4);

        if result == 10 {
            Ok(())
        } else {
            Err("6 + 4 != 10".to_string())
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 5,
        };

        let smaller = Rectangle {
            width: 8,
            height: 2,
        };

        // assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    // 自定义失败信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("fengmuxia");
        assert!(
            result.contains("fengmuxia"),
            "Greeting did not contains name, it's {result}"
        );
    }

    // 使用should_panic检查运行代码中是否有panic
    #[test]
    #[ignore] // 忽略该测试
    #[should_panic(expected = "is or no contains name")]
    fn greeting_contains_name2() {
        let result = greeting("heitiejingjie");

        assert!(result.contains("heitiejingjie"));
    }
}
