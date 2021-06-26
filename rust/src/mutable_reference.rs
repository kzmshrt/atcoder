fn main() {
    reference_of_mutable_variable();
    lifetime_of_reference();
    multiple_borrow();
    mutable_variable_of_reference_type();
    mutable_reference();
}

///
/// - 元の参照が可変であっても、参照を介してこれを書き換えることはできない
/// - 最後の行はコンパイルエラーになる
///
fn reference_of_mutable_variable() {
    let mut foo = 10;
    let reference = &foo;
    assert_eq!(*reference, 10);
    // *reference = 20;
}

///
/// - 'cannot assign to `foo` because it is borrowed'
/// - `foo` が借用されているので、`foo` に対する代入はできない
///
fn lifetime_of_reference() {
    // 1
    // let mut foo = 10;
    // let reference = &foo;
    // println!("{}", reference);
    // foo = 20;
    // println!("{}", reference);

    // 2
    let mut foo = 10;
    let reference = &foo;
    println!("{}", reference); // &foo のライフタイムが終了
    foo = 20; // foo への代入が可能
    println!("{}", foo);
}

///
/// - 可変変数に対して複数の借用が存在する場合は、すべての借用のライフタイムの終了後に、再代入が可能になる
///
fn multiple_borrow() {
    let mut foo = 10;
    let reference1 = &foo;
    let reference2 = &foo;
    assert_eq!(*reference1, 10);
    assert_eq!(*reference2, 10);
    foo = 20;
    println!("{}", foo);
}

///
/// # 参照型の可変変数
///
/// - `mut` を付けて参照型の変数を宣言すると、参照を複数回代入することができる
/// - 参照型の可変変数に新しい参照が代入されると、前の参照は使えなくなる
/// - 前の参照のライフタイムは、代入前の最後の使用で終了する
///
fn mutable_variable_of_reference_type() {
    let hoge = 10;
    let fuga = 20;
    let mut reference = &hoge;
    reference = &fuga;
    reference = &hoge;
    reference = &10;

    let mut reference;
    {
        let hoge = 10;
        reference = &hoge; // &hoge のライフタイム開始
        assert_eq!(*reference, 10); // &hoge のライフタイム終了
        reference = &20; // &hoge は無効
    }
    assert_eq!(*reference, 20); // &20 は静的ライフタイムなので有効
}

///
/// # 可変参照
///
/// - 可変変数は、参照の他に、「可変参照」をとることができる
/// - 「可変参照」に対して、可変でない参照を「不変参照」と呼ぶ
/// - 可変参照は、参照を外して中身を書き換えることが可能
/// - 可変参照の型は、`&T` ではなく、`&mut T`
///
/// # 可変性と借用に関する3つのルール
///
/// - 不変参照は参照先を書き換えることができない、可変参照は参照先を書き換えることができる
/// - 変数が借用されている間は、変数を書き換えたり可変として参照することができない
/// - 変数が可変として借用されている間は、変数を使用したり借用することができない
///
fn mutable_reference() {
    let mut hoge = 10;
    let reference: &mut i32 = &mut hoge;
    assert_eq!(*reference, 10);
    *reference = 20;
    assert_eq!(*reference, 20);

    // 可変参照も、不変参照と同じく、すべての借用のライフタイムが終了するまで、もとの変数に再代入することはできない
    // よって、以下のコードはコンパイルエラーになる
    // let mut hoge = 10;
    // let reference = &mut hoge;
    // println!("{}", reference);
    // hoge = 20;
    // println!("{}", reference);

    // もとの変数が可変でない場合は、可変参照をとることはできない
    // よって、以下のコードはコンパイルエラーになる
    // let hoge = 10;
    // let reference = &mut hoge;

    // 借用によって、もとの変数が可変でなくなっている場合は、可変参照をとることはできない
    // 可変として借用する前に、すべての他の借用のライフタイムが終了していなければならない
    // よって、以下のコードはコンパイルエラーになる
    // let mut hoge = 10;
    // let immutable_reference = &hoge;
    // let mutable_reference = &mut hoge;
    // println!("{}", immutable_reference);

    // 借用によって、もとの変数が可変でなくなっている場合は、可変参照をとることはできない
    // 可変として借用する前に、すべての他の借用のライフタイムが終了していなければならない
    // それは、前の借用が可変参照であったとしても同じ
    // よって、以下のコードはコンパイルエラーになる
    // let mut hoge = 10;
    // let mutable_reference_1 = &mut hoge;
    // let mutable_reference_2 = &mut hoge;
    // println!("{}", mutable_reference_1);

    // 可変として借用された変数は、いつ書き換えられるか分からない
    // そのため、可変参照のライフタイムが存続している間は、もとの変数の使用や借用ができない
    // よって、以下のコードはコンパイルエラーになる
    // let mut hoge = 10;
    // let reference = &mut hoge;
    // let fuga = hoge + 20;
    // *reference += 20;

    // &T型の変数を、&mut T型の変数に代入することはできない
    // よって、以下のコードはコンパイルエラーになる
    // let mut hoge: i32 = 10;
    // let immutable_reference = &hoge;
    // let mutable_reference: &mut i32 = immutable_reference;

    // &mut T型の変数を、&T型の変数に代入することはできる = 「型強制」
    let mut hoge: i32 = 10;
    let mutable_reference: &mut i32 = &mut hoge;
    let immutable_reference: &i32 = mutable_reference;
    assert_eq!(*immutable_reference, 10);

    // 可変参照のパターンマッチ
    let mut hoge = 10;
    let &mut copied = &mut hoge;
    // let &mut copied = &hoge; エラー
    // let &copied = &mut hoge; エラー
    println!("{}", copied);

    // ref パターン（復習）
    // 1 と 2 は同じ
    // 1
    let mut hoge = 10;
    let reference = &hoge;
    assert_eq!(*reference, 10);
    // 2
    let mut hoge = 10;
    let ref reference = hoge;
    assert_eq!(*reference, 10);

    // ref mut パターン
    // 1 と 2 は同じ
    // 1
    let mut hoge = 10;
    let reference = &mut hoge;
    *reference += 10;
    assert_eq!(*reference, 20);
    // 2
    let mut hoge = 10;
    let ref mut reference = hoge;
    *reference += 10;
    assert_eq!(*reference, 20);
    // 3
    // ref と mut の順番を逆にすることはできない
    // よって、以下のコードはコンパイルエラーになる
    // let mut hoge = 10;
    // let mut ref reference = hoge;

    // &mut と ref mut の省略
    // - &(ref x, ref y) は (x, y) と省略できる
    // - &(ref mut x, ref mut y) も (x, y) と省略できる
    let mut tuple = (3, 10);
    let (x, y) = &mut tuple;
    *x *= *y;
    assert_eq!(tuple, (30, 10));

    // for 式
    let mut array = [10, 20, 30];
    for i in &mut array {
        *i += 1;
    }
    assert_eq!(array, [11, 21, 31]);
}
