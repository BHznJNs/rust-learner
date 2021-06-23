// Output formatly | 格式化输出
fn main() {
    /*
        format! : Output text which was formated
        print! : Just like format!, but it can output text into console
        println! : Just like print!, but a '\n' is appended to the end
        eprint! : Just like format!, but it can output text into standard error
        eprintln! : Just like eprint!, but a '\n' is appended to the end
        ————————————————————————————————————————
        format! : 输出格式化之后的文本
        print! : 与 format! 一样能格式化文本，但会将文本打印到控制台
        println! : 在 print! 基础上，附加了换行符
        eprint! : 与 format! 一样能格式化文本，但会将文本输出到标准错误（？）
        eprintln! : 在 eprint! 基础上，附加了换行符
    */

    //————————————————————————————————————————

    /*
        You can output text which was formated by format!()
        你可以使用 format 函数输出格式化之后的文本
    */
    let num = 123;
    println!("{}", format!("{}", num));
    // format!("{}", num)
    println!("{}", format!("{:b}", num));
    // format!("0o{:b}", num) => 1111011 (2进制)
    println!("{}", format!("0o{:o}", num));
    // format!("0o{:o}", num) => 0o173 (8进制)
    println!("{}", format!("0x{:X}", num));
    // format!("0x{:X}", num) => 0x7B (16进制)

    //————————————————————————————————————————

    /*
        You can format the output of println.
        In the following sentence, the number 10 will replace "{}" in it
        你可以通过格式化输出语法来格式化打印。
        在下面的语句中，数字 10 会代替字符串中的 {}。
    */
    println!("This number is {}", 10);
    // This number is 10

    /*
        You can use boolean logic as an argv.
        你可以使用布尔逻辑判断作为一个参数。
    */
    let num1 :i8 = 1;
    let num2 :i8 = 2;
    println!("The num2 is bigger num1: {}", num2 > num1);
    // The num2 is bigger num1: true

    /*
        You can replace several {} by input several argv.
        你可以通过传入多个参数来实现对多个 {} 进行替换。
    */
    println!("This number is {} and that number is {}", 10, 20);
    // This number is 10, 这个数字是 20

    //————————————————————————————————————————

    /*
        You can assign an argv by using index (It will be
            an error when you used an missing argv).
        你可以通过索引来指定参数（当使用了不存在的参数时会报错）。

        Error Exam:
            println!("This number is {1} and that number is {0}", 10);
    */
    println!("This number is {1} and that number is {0}", 10, 20);
    // This number is 20 and that number is 10

    /*
        You can also add an index to reuse one argv.
        你也可以在 {} 中添加索引来重复使用同一个参数。
    */
    println!("This number is {0} and that number is also {0}", 10);
    // This number is 10 and that number is also 10


    /*
        You can name the argv.
        你可以给参数命名（这个语法糖不错）。
    */
    println!("This argv's name is {name}", name="Flex");
    // This argv's name is Flex
}