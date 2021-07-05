///
/// # コピー可能性
///
/// - コピー可能
///     − 数値型
///         − i32
///         - usize
///         - f64
///     − &T 不変参照
/// − コピー不可能
///     - Vec<T> ベクタ
///     - String 文字列
///     - &mut T 可変参照
///
fn main() {
    practice_move();
    practice_borrow();
}

fn practice_move() {
    // コピー可能な場合
    let hoge: i32 = 10; // 変数 hoge が値10に対して所有権を持つ
    let copied = hoge; // メモリ上に別の4バイトの領域が確保され、そこに変数 hoge の値10がコピーされる

    // コピー不可能な場合
    let mut vector: Vec<i32> = vec![1, 2, 3];
    let moved = vector; // メモリ上に vec![1, 2, 3] の領域が新たに確保されるのではなく、所有権だけが vector から moved に移動する

    // 所有権を得た変数 moved は使える
    println!("{:?}", moved);

    // 所有権を失った変数 vector は使えない
    // println!("{:?}", vector);

    // 所有権を失っても変数 vector のスコープは終了しない
    // 可変な変数なら、新しい値を代入することで再度使用可能となる
    vector = vec![4, 5, 6];
    println!("{:?}", vector);
}

fn practice_borrow() {
    // 借用はムーブではない
    let vector: Vec<i32> = vec![1, 2, 3];
    let reference = &vector;

    // vector の所有権はムーブされていないので使用可能
    println!("{:?}", vector);
    println!("{:?}", reference);
}
