fn main() {
    practice_print();
    practice_print_with_order();
    practice_print_named();
    practice_print_format();
}

fn practice_print() {
    let a = 120;
    let b = 450;
    println!("{} + {} = {}", a, b, a + b);
}

fn practice_print_with_order() {
    println!("{} {} {} {} {}", -2, 10, 2.4, 30, 4.5);
    println!("{2} {3} {0} {4} {1}", -2, 10, 2.4, 30, 4.5);
    println!("{0} {1} {0} {1}", -2, 10);
}

fn practice_print_named() {
    println!("{foo} {bar}", bar = -2, foo = 10);
}

fn practice_print_format() {
    println!("{:6} {:4}", 81, -79); // 桁数指定
    println!("{1:6} {0:4}", 81, -79); // 順番と桁数指定の両立
    println!("{:06} {:04}", 81, -79); // 0埋め桁数指定
    println!("{:<6} {:<4}", 81, -79); // 左寄せ
    println!("{:^6} {:^4}", 81, -79); // 中央寄せ
    println!("{:>6} {:>6}", 81, -79); // 右寄せ（一応あった）
    println!("{:+} {:+}", 81, -79); // 符号明示
    println!("{:b} {:b}", 81, -79); // 2進法
    println!("{:o} {:o}", 81, -79); // 8進法
    println!("{:x} {:x}", 81, -79); // 16進法（小文字）
    println!("{:X} {:X}", 81, -79); // 16進法（大文字）
    println!("{:.6} {:.4}", 81.12345678, -79.12345678); // 小数点以下指定桁数まで（四捨五入）
    println!("{:e} {:e}", 8112345678.0, -7912345678.0); // 指数表記（小文字）
    println!("{:E} {:E}", 8112345678.0, -7912345678.0); //指数表記（大文字）
    println!("{:?}", (10, 20, 30)); // タプル
    println!("{:?}", [10, 20, 30]); // 配列
    println!("{:#?}", (10, 20, 30)); // タプル（改行あり）
    println!("{:#?}", [10, 20, 30]); // 配列（改行あり）
}
