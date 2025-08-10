pub mod type_test {
    pub fn display() {
        bool_demo();
        println!("=====================");
        char_demo();
        println!("=====================");
        int_demo();
        println!("=====================");
        int_overflow();
        println!("=====================");
        type_transform();
    }

    fn bool_demo() {
        let x = true;
        let y: bool = !x; // 取反操作

        let z = x && y; // 逻辑与操作,带短路功能
        println!("{z}");

        let z = x || y; // 逻辑或操作,不带短路操作
        println!("{z}");

        let z = x & y; // 按位与操作，不带短路功能
        println!("{z}");

        let z = x | y; // 按位或操作，不带短路功能
        println!("{z}");

        let z = x ^ y; // 按位异或操作，不带短路功能
        println!("{z}");
    }

    fn char_demo() {
        let love = "❤️"; // 可以嵌入任何unicode字符 
        println!("{love}");

        let x: u8 = 1;
        let y: u8 = b'A';
        let z: &[u8; 5] = b"hello";
        let a: &[u8; 14] = br#"hello \n world"#;
        println!("{x}, {y}, {:?}, {:?}", z, a);
    }

    fn int_demo() {
        let var1: i32 = 32;
        let var2: i32 = 0xff; // 十六进制
        let var3: i32 = 0o55; // 八进制
        let var4: i32 = 0b1001; // 二进制
        println!("{var1}, {var2}, {var3}, {var4}");
    }

    fn int_overflow() {
        // 整数溢出
        let m: i8 = 120;
        let n: i8 = 120;

        // println!("{}", m + n);
    }

    fn float_demo() {}

    fn type_transform() {
        // rust中任何类型的转换必须显式指示
        let x: i8 = 12;
        // let y: i16 = x;
        let _t_y: i16 = x as i16;

        // 有时需要多个as才能转换成功
        let a = 29;
        let p_a = &a;
        println!("{:p}", p_a);
    }
}
