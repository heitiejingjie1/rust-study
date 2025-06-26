fn main() {
    str_to_string();
    string_to_str();
    str_push();
    str_insert();
    str_repalace();
    str_delete();
    str_concatenate();
    str_operator();
}

fn str_to_string() {
    // 使用to_string()方法和string结构体from()方法
    let s = String::from("接天连日无穷碧");
    println!("{}", s);
    let s2 = "映日荷花别样红".to_string();
    println!("{}", s2);
}

fn string_to_str() {
    // 使用as_str()方法和取string结构体的引用
    let s = String::from("关关雎鸠,在河之洲");
    let s2 = s.as_str();
    println!("{}", s2);

    let _s3 = &s; // 取string结构体的引用
}

/*
* 无法使用字符串索引获取字符,不清楚字符的字节数
*/

fn str_push() {
    // 在原有字符串上追加字符或字符串，不会返回新的字符串
    let mut s = String::from("采菊东篱下");
    s.push(','); // 追加字符
    s.push_str("悠然见南山."); // 追加字符串 
    println!("{}", s);
}

fn str_insert() {
    // 在指定位置插入字符或字符串，不会返回新的字符串
    let mut s = String::from("人生不相见");
    let mut len = s.len();
    s.insert(len, ',');
    len += 1;
    s.insert_str(len, "动如参与商。");
    println!("{}", s);
}

fn str_repalace() {
    // repalace()方法，替换匹配到的所有字符串，适用于string类型和&str类型，返回一个新的字符串，无需设置mut参数
    let s = String::from("张学友，张学友，我们爱你。我爱黎明，我爱黎明。");
    let s2 = s.replace("张学友", "薛之谦");
    println!("{}", s2);

    // replacen()方法，替换指定个数的字符串
    let mut s3 = s2.replacen("薛之谦", "刘德华", 1);
    println!("{}", s3);

    // replace_range()方法，替换指定范围的字符串，操作原字符串，需设置mut参数
    s3.replace_range(0..3, "张");
    println!("{}", s3);
}

fn str_delete() {
    // pop()，删除并返回字符串的最后一个字符，如果字符串为空，返回none
    let mut s = String::from("人生若只如初见");
    let last_char = s.pop(); // 返回值为Option类型
    println!("{:?}", last_char);

    // remove(), 删除字符串中指定位置的字符，有返回值
    let some_char = s.remove(6);
    println!("{}", some_char);

    // truncate(), 删除字符串中指定位置之后的所有字符
    s.truncate(9);
    println!("{}", s);

    // clear(), 清空字符串
    s.clear();
}

fn str_concatenate() {
    // 使用"+"运算符，相当于调用std::string标准库的add()
    // 或format！宏来连接字符串

    let s1 = String::from("草长莺飞二月天,");
    let s2 = "拂堤杨柳醉春烟。".to_string();

    let s3 = s1 + &s2; // 注意这里的s2需要转换为&str类型,s1的所有权被转移到s'3中 '
    println!("{}", s3);
    // println!("{}", s1);

    // 使用format!宏连接字符串，不会转移所有权
    let s4 = format!("{}{}", s3, "儿童放学归来早，忙趁东风放纸鸢。");
    println!("{}", s4);
}

fn str_operator() {
    // 以字符形式操作字符串
    let s = "墙角数枝梅，凌寒独自开".to_string();
    for c in s.chars() {
        println!("{}", c);
    }

    // 以字节形式操作字符
    for c in s.bytes() {
        println!("{}", c);
    }
}
