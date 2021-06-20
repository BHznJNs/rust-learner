// Hello World！
fn main() {
    // This is a comment | 这是一个注释。
    /* This is also a comment | 这也是一个注释。 */
    println!("hello world!");
    /*     ^                ^
             注意这个感叹号！    注意这个分号！
           Be aware that the exclamation symbol and semicolons!
        你需要在不是最后一个语句的语句末尾添加分号！
        You need to add a semicolon to a sentence end if it's not the last
    */
    println!("hello world!")
    /*                      ^
        最后一个语句不添加分号不会报错。
        it will not be an error if you didn't add a semicolon to the last
    */
}
/*
    The result | 输出结果：
    hello world!
    hello world!
*/
