fn main() {
    practice_generics_less();
    practice_generics();
    practice_trait();
}

fn practice_generics_less() {
    fn second_f64_i32((_, x): (f64, i32)) -> i32 {
        x
    }
    fn second_f32_i32((_, x): (f32, i32)) -> i32 {
        x
    }
    fn second_bool_i32((_, x): (bool, i32)) -> i32 {
        x
    }

    assert_eq!(second_f64_i32((5., 3)), 3);
    assert_eq!(second_f32_i32((3., 3)), 3);
    assert_eq!(second_bool_i32((true, 3)), 3);
}

fn practice_generics() {
    // 型パラメータ
    fn second_i32<T>((_, x): (T, i32)) -> i32 {
        x
    }

    assert_eq!(second_i32::<f64>((3_f64, 5)), 5);
    assert_eq!(second_i32::<f32>((3_f32, 5)), 5);
    assert_eq!(second_i32::<bool>((true, 5)), 5);

    fn second<T, U>((_, x): (T, U)) -> U {
        x
    }

    assert_eq!(second::<f64, i32>((3_f64, 5_i32)), 5);

    // 型パラメータを '_' で省略することで、型推論を使える
    let result = second::<bool, _>((true, 65));
    assert_eq!(result, b'A');

    let result = second::<_, _>((true, 65));
    assert_eq!(result, b'A');

    // 全ての型パラメータが推論可能な状況下では、型パラメータ不要
    let result = second((true, 65));
    assert_eq!(result, b'A');
}

fn practice_trait() {
    // 「全ての型 T について」ではなく「{} で出力できるような全ての型 T について」の関数を書く
    // = 型 T についての条件
    // = トレイト
    // → 「T がトレイトを満たす」という制約 = トレイト境界
    fn print<T: std::fmt::Display>(x: T) {
        println!("{}", x);
    }

    print(10);
    print("Hello");
}
