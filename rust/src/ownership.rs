///
/// # コピー可能性
///
/// - コピー可能
///     − 数値型
///         − `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
///         - `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
///         - `f32`, `f64`
///         - `char`
///         - `bool`
///     − `&T` 不変参照
///     − コピー可能な型 `T` に対する配列 `[T; N]`
///     - コピー可能な型 `T`, `U`, ... に対するタプル `(T, U, ...)`
/// − コピー不可能
///     - `Vec<T>` ベクタ
///     - `String` 文字列
///     - `&mut T` 可変参照
///
fn main() {
    practice_move();
    practice_borrow();
    practice_drop();
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

fn practice_drop() {
    // hoge が宣言されていたブロックが終了すると、変数 hoge のスコープが終了する、そして変数 hoge の所有権のために確保されていたメモリ領域は解放され、未使用の状態に戻る
    // これを値の「ドロップ」と呼ぶ
    {
        let hoge = 10;
    }

    {
        let hoge = 10; // first 10
        {
            let fuga = hoge; // second 10
        } // drop value of fuga
    } // drop value of hoge

    // ドロップと所有権
    // 変数のスコープの終了時にドロップされるのは、その変数が所有していた値である
    // 変数が「所有」していない場合は、ドロップは起こらない

    // 内側ブロック内でベクタの所有権が vector から moved に移動する
    // 内側ブロックの終了時、moved のスコープが終了し、moved が所有していたベクタの値がドロップされる
    // 外側ブロックの終了時、vector のスコープが終了するが、vector は何も所有していないためドロップは起こらない
    // 「値の所有者は常に一つ」という規則により、ドロップも一つの値に対して一度だけ発生する
    {
        let vector = vec![10, 20, 30];
        {
            let moved = vector;
        } // drop value of moved
    }
}
