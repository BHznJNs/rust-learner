fn main() {
    /*
        An variable is default immutable.
        变量默认是不可变的。
    */
    let immutable_num = 1;
    /*
        If you attempt to change an immutable variable, just like this.
        如果你试图改变一个不可变的变量，会出现类似下面的报错。

        Error Exam:
        let immutable_num = 1;
        immutable_num = 2;
        ^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
        error: aborting due to previous error; 1 warning emitted
    */
    println!("immutable_num: {}", immutable_num);
    // immutable_num: 1

    /*
        If you need to change an variable, add a "mut" behind the "let".
        如果你需要改变一个变量的值，在 “let” 后加上 “mut”。
    */
    let mut _mutable_num = 1;
    _mutable_num = 2;
    println!("_mutable_num: {}", _mutable_num);
    // _mutable_num: 2

    /*
        You can use the "const" to define a constant.
        你可以用 const 关键字来定义一个常量。
    */
    const PI :f64 = 3.14;
    /*       ^^^^
        Be aware that you must always annotate the type to a constant.
        注意！对于常量你必须标注其类型。

        Error Exam:
        const PI = 3.14;
              ^^ help: provide a type for the item: `PI: f64`
        error: aborting due to previous error
    */
    println!("PI: {}", PI);
    // PI: 3.14

    /*
        Be aware of the difference between an immutable variable and a constant,
        注意！不可变变量与常量的区别在于，
        you can change the value of an immutable variable by the "let", which is also named shadow.
        你可以使用 let 关键字来改变（或者说重新定义）不可变变量的值，这又被称为变量遮蔽。
        However, you can't change the value of a constant by any way.
        但你无法通过任何方法改变常量的值。
    */
    let immutable_num = 2;
    println!("immutable_num: {}", immutable_num);
    // immutable_num: 2
    /*
        Error Exam:
        PI = 3.14159
        -- ^
        cannot assign to this expression
        error: aborting due to previous error

        const PI :f64 = 3.14159;
              ^^ help: provide a type for the item: `PI: f64`
        note: `PI` must be defined only once in the value namespace of this block 
        error: aborting due to previous error
    */
}