///
/// # スライス
///
/// - 配列とベクタは別物
/// - 配列とベクタには共通点もある
///     - 同じ型の値が連続で並んでいる
///     - 要素の個数を知ることができる
///     - `[i]` で `i` 番目の要素にアクセスできる
///     - `for` で走査できる
/// - 配列とベクタの共通部分をまとめて扱うための型がスライス
///
fn main() {
    // slice は常に参照として使用しなければならない特別な型
    // &[T] や &mut [T] でしか宣言できない
    // let slice: [i32];
    let ref_slice: &[i32];

    // 配列への参照 &[T; N] は「スライスへの参照」として扱える
    let mut ref_slice: &[i32];
    let array = [1, 2, 3];
    ref_slice = &array;
    println!("{:?}", ref_slice);
    // ベクタへの参照 &Vec<T> も「スライスへの参照」として扱える
    let vector = vec![4, 5, 6];
    ref_slice = &vector;
    println!("{:?}", ref_slice);

    // スライスの参照は、配列やベクタと同じく、ライフタイムがもとの変数のスコープを超えてはならない
    // let ref_slice: &[i32] = {
    //     let array = [1, 2, 3];
    //     &array;
    // };

    // スライスは .. operator によりサブスライスを得ることができる
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let ref_slice = &array[1..4];
    println!("{:?}", ref_slice);
    // a..=b もできる
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let ref_slice = &array[1..=4];
    println!("{:?}", ref_slice);
    // a.. もできる
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let ref_slice = &array[1..];
    println!("{:?}", ref_slice);
    // ..b もできる
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let ref_slice = &array[..4];
    println!("{:?}", ref_slice);
    // .. もできる → 全範囲
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let ref_slice = &array[..];
    println!("{:?}", ref_slice);
    // 空のスライス
    let empty = &[0, 1, 2, 3, 4][2..2];
    println!("{:?}", empty);

    // for
    let array = [0, 1, 2, 3, 4];
    for i in &array[..] {
        println!("{}", i);
    }

    // swap
    // 可変なスライスにたいして x.swap(a, b) は x[a] と x[b] を入れ替える
    let mut array = [0, 1, 2, 3, 4];
    let ref_mut_slice = &mut array[..];
    ref_mut_slice.swap(1, 3);
    println!("{:?}", array);
    // x.swap(a, b) と書いた時 x が配列やベクタであればスライスへと型強制される
    let mut array = [0, 1, 2, 3, 4];
    array.swap(1, 3);
    println!("{:?}", array);

    // reverse
    // 可変なスライスを逆順にする
    let mut array = [7, 2, -3, 9, -2, 5];
    array.reverse();
    assert_eq!(array, [5, -2, 9, -3, 2, 7]);

    // sort
    // 可変なスライスを小さい順に並べ替える
    let mut array = [7, 2, -3, 9, -2, 5];
    array.sort();
    assert_eq!(array, [-3, -2, 2, 5, 7, 9]);
}
