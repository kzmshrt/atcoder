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
}
