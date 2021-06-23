fn main() {
    div(55);
    div_euclid(-123);
}

fn div(x: i32) {
    let r = x % 10;
    assert!(0 <= r && r < 10, "割った余りが想定の範囲から外れています");
    println!("あまりは {}", r);
}

fn div_euclid(x: i32) {
    // 負の数のあまりを数学に沿うように計算する
    let r = x.rem_euclid(10);
    assert!(0 <= r && r < 10);
    println!("あまりは {}", r);
}
