///
/// # 論駁可能なパターンと論駁不可能なパターン
///
/// - 論駁可能なパターン = 失敗する可能性のあるパターン
/// - 論駁不可能なパターン = 必ず成功するパターン
///
fn main() {
    // if let 式
    let ref_slice: &[i32] = &[10, 15, 20];
    if let [x, y, z] = *ref_slice {
        println!("{} {} {}", x, y, z);
    } else {
        println!("マッチに失敗しました");
    }

    let ref_slice: &[i32] = &[10, 15];
    if let [x, y, z] = *ref_slice {
        println!("{} {} {}", x, y, z);
    } else {
        println!("マッチに失敗しました")
    }
}
