fn main() {
    practice_mut();
    practice_compound_assignment_operator();
    practice_for_and_mutable();
    practice_proconio_mut();
}

fn practice_mut() {
    let mut mutable: i32;
    mutable = 20;
    println!("{}", mutable);
    mutable = 30;
    println!("{}", mutable);
    // 型を変更することはできない
    // 以下のコードはコンパイルエラー
    // mutable = 40_f64;

    // 可変変数への再代入ではアドレスは変更されない
    let mut mutable: i32;
    mutable = 20;
    println!("{:p}", &mutable);
    mutable = 30;
    println!("{:p}", &mutable);
}

fn practice_compound_assignment_operator() {
    let mut mutable = 100;
    println!("{}", mutable);
    mutable -= 10;
    println!("{}", mutable);
    mutable *= 2;
    println!("{}", mutable);
    mutable /= 3;
    println!("{}", mutable);
    mutable %= 7;
    println!("{}", mutable);
    mutable += 4;
    println!("{}", mutable);
}

///
/// - `sum += num;` では `+=` が `num` の参照外しを行っている
/// - 1 は 2 のようにも書ける
///
fn practice_for_and_mutable() {
    // 1
    let array = [30, 20, 50];
    let mut sum = 0;
    for num in &array {
        sum += num;
    }
    assert_eq!(sum, 100);

    // 2
    let array = [30, 20, 50, 10, 90];
    let sum: i32 = array.iter().sum();
    assert_eq!(sum, 200);
}

fn practice_proconio_mut() {
    proconio::input! {
        mut a: i32
    }
    a += 20;
    println!("{}", a);
}
