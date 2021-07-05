fn main() {
    // String
    // 文字列処理における Vec<T> に相当する型
    let s: String;
    let s = String::new();

    // str
    // 文字列処理におけるスライス [T] に相当する型
    let s = String::new();
    let slice: &str = &s;
    // 文字列リテラル自体の型も &str
    // 文字列リテラルは、他のリテラルへの参照と同様に、静的なライフタイムをもつ
    let slice = "Hello";

    // &str -> String
    let string = "Hello".to_string();
    let string = String::from("Hello");

    // String や &str は println("{}", s) で出力できる
    let greeting = "Hello";
    let world = "world".to_string();
    println!("{}, {}!", greeting, world);

    // println! マクロでは、フォーマット文字列として必ず文字列リテラルを渡す必要がある
    // let s = "Hello";
    // println!(s);

    // chars
    // s.chars() で一文字ずつしょりできる
    let s = "Hello";
    for c in s.chars() {
        println!("{}", c);
    }
    // char 型リテラルは '' （シングルクォート）で書く
    let s = "打打打打打打打打打打";
    for c in s.chars() {
        assert_eq!(c, '打');
    }

    // bytes
    // 文字列の中身は u8 型の値のシーケンス
    // s.bytes() で u8 型の値を一つずつ処理できる
    let s = "𠮷野家で𩸽";
    for c in s.bytes() {
        println!("{:x}", c);
    }

    // to_string
    // 値を String 型に変換する
    let x: i32 = 10;
    assert_eq!(x.to_string(), "10".to_string());
    let x: f64 = 120.0;
    assert_eq!(x.to_string(), "120".to_string());
    let x: char = 'A';
    assert_eq!(x.to_string(), "A".to_string());

    // format!
    let s = format!("{} {}", 10, 2.5);
    assert_eq!(s, "10 2.5".to_string());

    // byte literal
    let c = b'A';
    println!("{:x}", c);

    // byte string literal
    let array = b"Hello";
    assert_eq!(*array, [b'H', b'e', b'l', b'l', b'o']);
}
