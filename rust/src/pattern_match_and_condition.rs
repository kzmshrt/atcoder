///
/// # 論駁可能なパターンと論駁不可能なパターン
///
/// - 論駁可能なパターン = 失敗する可能性のあるパターン
/// - 論駁不可能なパターン = 必ず成功するパターン
///
fn main() {
    // if let マッチ成功
    let ref_slice: &[i32] = &[10, 15, 20];
    if let [x, y, z] = *ref_slice {
        println!("{} {} {}", x, y, z);
    } else {
        println!("マッチに失敗しました");
    }

    // if let マッチ失敗
    let ref_slice: &[i32] = &[10, 15];
    if let [x, y, z] = *ref_slice {
        println!("{} {} {}", x, y, z);
    } else {
        println!("マッチに失敗しました")
    }

    // if let リテラルパターン
    let vector: [(i32, i32); 5] = [(5, 1), (1, 0), (2, 1), (1, 2), (0, 0)];
    for &tuple in &vector {
        if let (1, value) = tuple {
            println!("{}", value);
        } else if let (2, value) = tuple {
            println!("{}", value);
        } else if let (0, 0) = tuple {
            break;
        } else {
            println!("?");
        }
    }

    // if let 複数パターン
    let array = [(1, 92), (3, 91), (95, 1), (94, 2)];
    let mut vector = Vec::new();
    for tuple in &array {
        if let (1, value) | (value, 2) = *tuple {
            vector.push(value);
        }
    }
    assert_eq!(vector, vec![92, 94]);

    // if let 複数パターン パターン間で変数の有無が一致しない場合はエラーになる
    // variable `y` is not bound in all patterns E0408 pattern doesn't bind `y`
    // let tuple = (3, 2, 1);
    // if let (x, 0, 0) | (x, y, 1) | (x, y, 2) = tuple {
    //     println!("{} {}", x, y);
    // }

    // if let 複数パターン パターン間で変数の型が一致しない場合はエラーになる
    // mismatched types E0308 expected `f64`, found `i32`
    // let tuple: (i32, f64) = (1, 2.0);
    // if let (1, x) | (x, 2.0) = tuple {
    //     println!("{}", x);
    // }

    // range
    // a..=b を用いてある範囲にマッチするパターンを表現できる
    let tuple = (1, 2);
    if let (0..=5, x) = tuple {
        assert_eq!(x, 2);
    } else {
        panic!();
    }
    // 現状は a..b は使用できない
    // exclusive range pattern syntax is experimental E0658 Note: for more information, see https://github.com/rust-lang/rust/issues/37854
    // let tuple = (1, 2);
    // if let (0..5, x) = tuple {
    //     assert_eq!(x, 2);
    // } else {
    //     panic!();
    // }

    // wild card
    // 3要素のうちどれか1つが1であるかを確認する
    let tuple = (3, 1, 2);
    if let (1, _, _) | (_, 1, _) | (_, _, 1) = tuple {
        println!("少なくとも1つが1");
    }
    // for で繰り返す回数だけを指定したいときにも使える
    for _ in 0..4 {
        println!("Knock, knock, knockin' on heaven's door");
    }

    // rest
    // [first, second, ..] はスライスの長さが2以上であればマッチ成功、1以下であればマッチ失敗
    let ref_slice: &[i32] = &[10, 20, 30];
    if let [first, second, ..] = *ref_slice {
        println!("first element: {}", first);
        println!("second element: {}", second);
    } else {
        println!("長さ 1 以下のスライス");
    }
    // [first, .., last]
    let ref_slice: &[i32] = &[10, 20, 30];
    if let &[first, .., last] = ref_slice {
        assert_eq!(first, 10);
        assert_eq!(last, 30);
    } else {
        panic!();
    }
    // [.., second_to_last, last]
    let ref_slice: &[i32] = &[10, 20, 30];
    if let &[.., second_to_last, last] = ref_slice {
        assert_eq!(second_to_last, 20);
        assert_eq!(last, 30);
    } else {
        panic!();
    }

    // while let
    let array = [0, 0, 0, 1, 2];
    let mut ref_slice = &array[..];
    while let [0, ..] = *ref_slice {
        ref_slice = &ref_slice[1..];
        println!("{:?}", ref_slice);
    }
    assert_eq!(ref_slice, [1, 2]);

    // match
    // これは
    let vector: [(i32, i32); 5] = [(5, 1), (1, 0), (2, 1), (1, 2), (0, 0)];
    for &tuple in &vector {
        if let (1, value) = tuple {
            println!("{}", value);
        } else if let (2, value) = tuple {
            println!("{}", value * value);
        } else if let (0, 0) = tuple {
            break;
        } else {
            println!("?");
        }
    }
    // こう書ける
    let vector: [(i32, i32); 5] = [(5, 1), (1, 0), (2, 1), (1, 2), (0, 0)];
    for &tuple in &vector {
        match tuple {
            (1, value) => println!("{}", value),
            (2, value) => println!("{}", value * value),
            (0, 0) => break,
            _ => println!("?"),
        }
    }

    // match の返す値の型は全て同じでなければならない
    // let x = 0;
    // let y = match x {
    //     0 => 2i32,
    //     _ => 2.5f64,
    // };

    // match guard
    let tuple = (1, 3);
    match tuple {
        (1, x) if x % 2 == 0 => println!("{}", x),
        _ => {}
    }

    // @ operator
    // パターンマッチの成功時にマッチした値を変数に代入して使うことができる
    let tuple = (1, 50);
    match tuple {
        (1, value @ 0..=9) => println!("1桁: {}", value),
        (1, value @ 10..=99) => println!("2桁: {}", value),
        (1, value @ 100..=std::i32::MAX) => println!("3桁以上: {}", value),
        (1, _) => println!("負"),
        _ => {}
    }
}
