fn main() {
    // 这是一个注释。
    /* 这也是一个注释。 */
    println!("hello world!");
    /*     ^ 注意这个感叹号 | ^ 注意这个分号！
     你需要在不是最后一个语句的语句末尾添加分号！ */
    println!("hello world!");
    /*                      ^
     当然，最后一个语句添加分号不会报错。*/
}
/*
    输出结果：
    hello world!
    hello world!
*/
