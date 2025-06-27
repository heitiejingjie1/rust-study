fn main() {
    array_demo();
}

fn array_demo() {
    /*
     * 数组：
     * 长度固定
     * 类型相同
     * 可以存储任意类型的值
     * */
    // 数组的长度是在编译时确定的
    let arr = [1, 2, 3, 4, 5];

    // 指定数组的长度和元素类型
    let arr2: [i64; 3] = [1, 2, 3];

    // 访问数组
    let first = arr[0];
    println!("The first element is: {}", first);
}
