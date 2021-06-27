fn main() {
    practice_vector();
    practice_proconio_vector();
}

fn practice_vector() {
    // 要素を指定して初期化
    let vector = vec![1, 2, 3];
    let vector = vec![1; 10];
    // 空のベクタの初期化
    let vector = Vec::<i32>::new();

    // 要素数
    let vector: Vec<i32> = vec![1, 2, 3];
    assert_eq!(vector.len(), 3_usize);

    // 型推論
    let vector = vec![1, 2, 3];
    // 3_i64 と比較されている → 各要素の型が i64 型である必要がある → vector の型は Vec<i64> と推論される
    assert_eq!(vector[2], 3_i64);

    // 要素の型が推論できる場合は Vec::new() と書ける
    let vector: Vec<i64>;
    vector = Vec::new();

    // 要素の追加
    // 可変としてベクタを宣言すると、要素を追加できる
    let mut vector = Vec::new();
    assert_eq!(vector.len(), 0);

    vector.push(10);
    vector.push(20);
    vector.push(30);
    assert_eq!(vec![10, 20, 30], vector);

    vector.push(40_i64);
    vector.push(50);
    assert_eq!(vec![10, 20, 30, 40, 50], vector);
}

fn practice_proconio_vector() {
    proconio::input!(n: usize, vec: [i32; n]);
    println!("{}", n);
    println!("{:?}", vec);
}

fn practice_atcoder_abc185_a_1() {
    proconio::input!(a: [i32; 4]);
    let mut ans = 100;
    for &i in &a {
        if ans > i {
            ans = i;
        }
    }
    println!("{}", ans);
}

fn practice_atcoder_abc185_a_2() {
    proconio::input!(a: [i32; 4]);
    let mut ans = 100;
    for &i in &a {
        ans = ans.min(i);
    }
    println!("{}", ans);
}

fn practice_atcoder_abc185_a_3() {
    proconio::input!(a: [i32; 4]);
    println!("{}", a.iter().min().unwrap());
}

fn practice_atcoder_abc181_b_1() {
    proconio::input!(n: usize, v: [(i64, i64); n]);
    let mut ans = 0;
    for &(a, b) in &v {
        for i in a..=b {
            ans += i;
        }
    }
    println!("{}", ans);
}

fn practice_atcoder_abc181_b_2() {
    proconio::input!(v: [(i64, i64)]);
    let mut ans = 0;
    for &(a, b) in &v {
        for i in a..=b {
            ans += i;
        }
    }
    println!("{}", ans);
}

fn practice_atcoder_abc181_b_3() {
    proconio::input!(v: [(i64, i64)]);
    println!(
        "{}",
        v.iter()
            .map(|(x, y)| (x + y) * (y - x + 1) / 2)
            .sum::<i64>()
    );
}
