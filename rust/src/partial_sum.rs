///
/// N 個の正の整数 a0, a1, ..., aN-1 と正の整数 W が与えられるとき、
/// a0, a1, ..., aN-1 の中から何個かの整数を選んで総和を W とすることができるかどうかを判定する
///
fn partial_sum(i: i32, w: i32, a: &Vec<i32>) -> bool {
    match i {
        0 => w == 0,
        _ => partial_sum(i - 1, w, a) || partial_sum(i - 1, w - a[i as usize - 1], a),
    }
}

fn main() {
    let (n, w) = (10, 51);
    let a = vec![5, 10, 42, 32, 13, 9, 22, 14, 4, 7];
    println!("{}", if partial_sum(n, w, &a) { "Yes" } else { "No" })
}
