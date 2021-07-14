fn main() {
    practice_binary_print();
    practice_bit_and();
    practice_bit_or();
    practice_bit_xor();
    practice_bit_not();
    practice_bit_shift();
}

fn practice_binary_print() {
    println!("u8");
    for i in 0..25_u8 {
        println!("{:08b}", i);
    }
    println!("i8");
    for i in -10..10_i8 {
        println!("{:08b}", i);
    }
}

fn practice_bit_and() {
    assert_eq!(-20_i8 & -70_i8, -88_i8);
}

fn practice_bit_or() {
    assert_eq!(-20_i8 | -70_i8, -2_i8);
}

fn practice_bit_xor() {
    assert_eq!(-20_i8 ^ -70_i8, 86_i8);
}

fn practice_bit_not() {
    assert_eq!(!25_i8, -26_i8);
}

fn practice_bit_shift() {
    // 左シフト
    assert_eq!(100i8 << 2, -112i8);

    // 右シフト
    // 論理シフト
    assert_eq!(150_u8 >> 2, 37_u8);
    // 算術シフト
    assert_eq!(50_i8 >> 2, 12_i8);
    assert_eq!(-50_i8 >> 2, -13_i8);
}
