fn main() {
    // 借用
    // - 変数 X への参照をとる == 「X を借用する」
    // - `T型` への参照は `&T型`
    let foo: i8 = 100;
    let reference: &i8 = &foo;
    println!("{:p}", reference); // フォーマット文字列（16進数アドレス）

    // 未初期化の変数を借用することはできない
    // 以下のコードはコンパイルエラーになる
    // let bar: i8;
    // let reference = &bar;

    // 参照外し
    // - メモリ上のアドレスに格納された変数 X の値を間接的に知ること == 「参照外し」
    // - 参照外しの演算子は `*`
    let foo: i8 = 100;
    let reference: &i8 = &foo;
    assert_eq!(*reference, 100_i8);

    // 型推論
    let foo = 100;
    let reference = &foo;
    assert_eq!(*reference, 100_i8);

    // 演算子の参照外し
    // 算術演算子（+, -, *, /, %）は、参照外しをしなくても、自動で参照外しを行ってから計算をしてくれる
    let foo: i8 = 100;
    let reference = &foo;
    assert_eq!(reference + 1_i8, 101_i8);
    // ↓ 同じこと
    let foo: i8 = 100;
    let reference = &foo;
    assert_eq!(*reference + 1_i8, 101_i8);

    // 参照型をフォーマット文字列にそのまま渡すと、自動で参照外しを行う
    let bar = 100;
    let reference = &bar;
    println!("{}", reference);

    // 参照もパターンマッチができる
    // copied の値は foo の値を間接的に受け取ってコピーしたにすぎないので、アドレスは異なる
    let foo = 100;
    let reference = &foo;
    let &copied = reference;
    assert_eq!(copied, 100);
    println!("foo:    {:p}", &foo);
    println!("copied: {:p}", &copied);
}
