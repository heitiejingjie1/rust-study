use std::io::SeekFrom;

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
        assert_ne!(result, 5);
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
}
