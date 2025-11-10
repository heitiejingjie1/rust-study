pub mod slice_demo {
    use std::usize;

    pub fn display() {
        slice_demo();
    }

    fn slice_demo() {
        let mut s = String::from("hello, world!");

        let word = first_word(&s);

        s.clear();
        println!("{word}");
    }

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    fn second_word(s: &String) -> (usize, usize) {}
}
