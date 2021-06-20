fn main() {
    // Bools | 布尔值
    let _true_ = true;
    let _false_ = false;
    /*
        Or:
        let _true_ :bool = true;
        let _false :bool = false;
    */

    // Integers | 整型
    /*
        i8 i16 i32 i64 i128 isize
        u8 u16 u32 u64 i128 usize
    */
    let _i8_ = 10i8; // Suffix annotation | 后缀说明
    // Or: let _i8_ :i8 = 10;
    println!("_i8_: {}", format!("{:o}", _i8_)); // _i8_: 12

    // Floating-point | 浮点数
    let _f64_ = 10.0; // Default as `f64` | 默认为 f64
    let _f32_ = 10.0f32; // Or let _f32_ :f32 = 10.0;
    println!("_f64_: {}", _f64_); // _f64_: 10
    println!("_f32_: {}", _f32_); // _f32_: 10

    // Charactor | 字符
    let _char_ :char = '你';
    /*                 ^
        注意这个引号！ | Be aware that quotation marks!
    */
    println!("_char_: {}", _char_); // 你

    // Tuple & Array | 元组和数组
    let _tuple_ = (1, 2.0, true, '你');
    let _array_ = [1, 2, 3, 4];
    // Elements in an array must be the same type! | 数组元素均为相同类型！
    /*
        Or:
        let _tuple_ :(i8, f64, bool, char) = (1, 2.0, true, '你');
        let _array_ :[i8; 4] = [1, 2, 3, 4];
    */
    println!("_tuple_: {:?}", _tuple_);
    println!("_array_: {:?}", _array_);
    /*                   ^
        You can output a tuple or an array by this.
        你可以用这个（{:?}）来输出元组和数组。
    */
}