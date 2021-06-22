fn main() {
    // 文字列リテラル
    println!("\"Fool,\" said I, \"you do not know\"");
    println!(
        "/\\\
\\/"
    );

    // 生文字列リテラル
    println!(r"\\\\\\\\\\");
    println!(r#""Fool," said I, "you do not know""#);

    // 整数リテラル
    let big_number: i64;
    big_number = 2147483648;
    println!("{}", big_number);
    let big_number_2;
    big_number_2 = 2147483648_i64;
    println!("{}", big_number_2);

    // 浮動小数点リテラル
    let avogadro_constant;
    avogadro_constant = 6.02e+23;
    println!("{}", avogadro_constant);
    let pi = 3.14_f32;
    println!("{}", pi);
}
