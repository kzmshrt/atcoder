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
    practice_unmovable();
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

fn practice_unmovable() {
    let vector: Vec<Vec<i32>> = vec![vec![2, 3, 4], vec![1], vec![0; 5]];
    for i in &vector[0] {
        println!("{}", i);
    }

    // for では in のあとに &[T; N] や &Vec<T> や &[T] を置くと、各要素への参照 &T が走査される
    // だから、これは可能
    for i in &vector {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
    // でも、これは不可能
    // data moved here
    // move occurs because `i` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
    // for &i in &vector {
    //     for j in i {
    //         print!("{} ", j);
    //     }
    //     println!()
    // }

    // Vec<T> の T がコピー可能でない場合、ベクタから要素だけをムーブすることはできない
    // 変数 vector は常に、「ベクタ全体への所有権を持っている」あるいは「一切所有権を持っていない」のどちらかの状態にしかなりえない
    // let vector: Vec<Vec<i32>> = vec![vec![2, 3, 4], vec![1], vec![0; 5]];
    // let moved = vector[0];

    // 参照
    // 変数が借用されている間はムーブできない
    // 借用中にムーブが起きると、参照の指す先がなくなってしまうため
    // let vector = vec![1, 2, 3];
    // let reference = &vector;     // 借用
    // let moved = vector;          // ムーブ不可能
    // println!("{:?}", reference); // ライフタイム終了

    // タプル
    // タプルの要素にコピー不可能なものが含まれている場合は、タプル自体もコピー不可能
    // let tuple: (String, f64) = ("hello".to_string(), 0.1); // String はコピー不可能
    // let moved = tuple; // ムーブ

    // タプルの作成時にもムーブが発生する
    // let hello = "hello".to_string();
    // let tuple: (String, f64) = (hello, 0.1); // hello がムーブされる
    // println!("{}", hello); // エラー

    // 部分的なムーブ
    // タプルの一部だけをムーブすることができる
    let tuple: (String, f64) = ("hello".to_string(), 0.1);
    let hello = tuple.0;
    // 一部の要素がムーブされても残りの要素は使用することができる
    assert_eq!(tuple.1, 0.1);
    // タプル全体を使用することはできなくなる
    // let reference = &tuple;

    // ワイルドカードパターン
    // `_` への代入は使用に含まれず、ムーブも発生しない
    let hello = "hello".to_string();
    let _ = hello;
    println!("{}", hello);

    // ブロックが返す値とムーブ
    // ブロックが値を返す際にもムーブが発生する
    let power_of_2 = {
        let mut v = vec![1];
        for i in 0..4 {
            v.push(v[i] * 2);
        }
        v
    };
    assert_eq!(power_of_2, vec![1, 2, 4, 8, 16]);

    // ループとムーブ
    // 1回目のループで、ベクタの値が vector から moved にムーブされる → moved はそのままスコープが終了し、ベクタをドロップしてしまう
    // 2回目のループで、所有権を持っていない vector を使用しようとすることになる → エラー
    // let vector: Vec<i32> = Vec::new();
    // for _ in 0..10 {
    //     let moved = vector;
    // }
    // 逆に、ループの中で vector に再び所有権を与えれば問題は発生しない
    let mut vector: Vec<i32> = Vec::new();
    for _ in 0..10 {
        let mut moved = vector;
        moved.push(0);
        vector = moved;
    }
    assert_eq!(vector, vec![0; 10]);
}
