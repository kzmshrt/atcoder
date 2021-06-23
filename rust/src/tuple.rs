fn main() {
    practice_tuple();
    practice_tuple_pattern_match();
    practice_tuple_returned_from_block(10, 4);
    practice_unit();
}

fn practice_tuple() {
    let tuple: (i32, f64, i32) = (10, 2.5, 20);
    println!("first element:  {}", tuple.0);
    println!("second element: {}", tuple.1);
    println!("third element:  {}", tuple.2);
}

fn practice_tuple_pattern_match() {
    let tuple = (10, 2.5);
    let (x, y) = tuple;
    assert_eq!(x, 10);
    assert_eq!(y, 2.5);

    // 宣言時にもパターンマッチできる
    let (c, g): (i32, f64) = (299792458, 6.67430e-11);
    assert_eq!(c, 299792458);
    assert_eq!(g, 6.67430e-11);
}

fn practice_tuple_returned_from_block(a: i32, b: i32) {
    // ブロックがタプルを返すこともできるので、複数の値を同時に返して、それを呼び出し元で直接展開することができる
    let (max, min) = if a > b { (a, b) } else { (b, a) };
    assert!(max >= min);
    println!("bigger one:  {}", max);
    println!("smaller one: {}", min);
}

fn practice_unit() {
    // Rust ではブロックの最後に式を書かない場合でもユニット "()" が返されている
    // ユニットは、型も "()" であり、値も "()" である
    let unit: ();
    unit = {
        println!("() を返すブロック");
    };
    assert_eq!(unit, ());
}
