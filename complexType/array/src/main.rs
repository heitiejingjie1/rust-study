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

    // println!("{}", arr2);
    for r in arr2.iter() {
        println!("{}", r);
    }

    let arr3 = [3; 10]; // 创建一个长度为10的数组，所有元素都为3 
    println!("Array with all elements as 3: {:?}", arr3);

    // 复杂类型数组
    let complex_arr: [String; 3] = std::array::from_fn(|_i| String::from("Hello"));
    println!("Complex array: {:?}", complex_arr);

    // 多维数组
    let multi_dimensional_array: [[i32; 2]; 3] = [[1, 2], [3, 4], [5, 6]];
    println!("Multi-dimensional array: {:?}", multi_dimensional_array);
}
