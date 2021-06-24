fn main() {
    let array: [i64; 5];
    array = [3, 7, 31, 127, 8191];
    assert_eq!(array[0], 3);
    assert_eq!(array[1], 7);
    assert_eq!(array[2], 31);
    assert_eq!(array[3], 127);
    assert_eq!(array[4], 8191);

    // 配列に渡すインデックスは usize 型
    let array = [1, 2, 3];
    assert_eq!(array[0usize], 1);
    assert_eq!(array[1], 2);

    // 値指定による配列の初期化
    let array = [57; 10];
    assert_eq!(array[0], 57);
    assert_eq!(array[4], 57);
    assert_eq!(array[9], 57);

    // 配列のパターンマッチ
    let [x, y, z] = [1, 2, 3];
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);
}
