fn main() {
    div(55);
    div_euclid(-123);
    round(10, 98);
}

fn div(x: i32) {
    let r = x % 10;
    assert!(
        0 <= r && r < 10,
        "割った余りが {} で、想定の範囲から外れています",
        r
    );
    println!("あまりは {}", r);
}

fn div_euclid(x: i32) {
    // 負の数のあまりを数学に沿うように計算する
    let r = x.rem_euclid(10);
    assert!(0 <= r && r < 10);
    println!("あまりは {}", r);
}

fn round(x: i32, y: i32) {
    let rounded = x / y * y;
    assert_eq!(rounded % y, 0);
}
