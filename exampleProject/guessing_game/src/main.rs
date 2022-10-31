use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    /*
        Create a random number between 1 and 10 by rand.
        使用 rand 生成一个 1 ～ 10 之间的随机数。
    */
    let secret_number = rand::thread_rng().gen_range(1..11);
    // println!("secret_number: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // A variable to save input of user | 储存用户输入

        io::stdin()
            .read_line(&mut guess).expect("Failed to read line.");
        /*   ^ 读行     ^
                    Means it is a reference
                    表示这个参数是一个引用（reference）
        */

        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        /*
            The trim method can elimilate whitespace at the begining and end of a string.
            trim 方法用来去除字符串开头和结尾的空白字符。
            The parse method parses a string into some kind of number(so you need tell Rust the exact number type).
            parse 方法用来将字符串解析成多种数字（所以你需要写明具体的数字类型）。
            Using match expression to resolve when the guess is NaN.
            使用 match 表达式当输入为非数字时的情况。
        */

        // Print number user inputted | 打印用户输入数字
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {println!("Too small!")},
            Ordering::Greater => {println!("Too big!")},
            Ordering::Equal => {
                /*
                    When the guess equals to the secret_number, break this loop.
                    当 guess 等于 secret_number 时，退出循环。
                */
                println!("It's right!");
                break;
            }
        }
        /*
            Compare guess with secret_number by a match expression.
            通过 match 表达式比较 guess 和 secret_number。
        */
    }
}
/*
Guess the number!
Please input your guess.
5
You guessed: 5
Too small!
Please input your guess.
9
You guessed: 9
Too big!
Please input your guess.
8
You guessed: 8
Too big!
Please input your guess.
6
You guessed: 6
Too small!
Please input your guess.
7
You guessed: 7
It's right!
*/
