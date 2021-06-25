fn main() {
    practice_pattern_match_tuple_reference();
    practice_ref();
}

fn practice_pattern_match_tuple_reference() {
    let elements: [(i32, f64); 5] = [(6, 12.0), (7, 14.0), (8, 16.0), (15, 31.0), (16, 32.1)];
    for &(number, weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }

    let &(number, weight) = &elements[0];
    println!("{}: {:.1}", number, weight);
}

///
/// # ref パターン
///
/// - 1 は 2 に同じ
fn practice_ref() {
    // 1
    let foo = 10;
    let ref reference = foo;
    assert_eq!(*reference, 10);

    // 2
    let foo = 10;
    let reference = &foo;
    assert_eq!(*reference, 10);

    // number には carbon.0 への参照が代入される
    // weight には carbon.1 への参照が代入される
    let carbon = (6, 12.0);
    let (ref number, ref weight) = carbon;
    assert_eq!(*number, 6);
    assert_eq!(*weight, 12.0);

    // 3
    let elements: [(i32, f64); 5] = [(6, 12.0), (7, 14.0), (8, 16.0), (15, 31.0), (16, 32.1)];
    for &(number, weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }

    // 4
    // println! マクロは参照を自動的に外すので、出力結果は 3 と 4 で同じになる
    let elements: [(i32, f64); 5] = [(6, 12.0), (7, 14.0), (8, 16.0), (15, 31.0), (16, 32.1)];
    for &(ref number, ref weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }

    // 5
    // - 4 のように「各要素に ref が付いたタプルへの参照」の形式をしたパターン」においては & を省略できる
    // - そして、先頭の & が省略されている状況下では、各要素の ref も省略できる
    // - したがって最終的に、4 と 5 は同じことを意味する
    let elements: [(i32, f64); 5] = [(6, 12.0), (7, 14.0), (8, 16.0), (15, 31.0), (16, 32.1)];
    for (number, weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }
}
