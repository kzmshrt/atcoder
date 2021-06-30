fn main() {
    // ---------- break 式 ----------
    let array = [2, 3, 0, 4, 5];
    for &i in &array {
        if i == 0 {
            break;
        }
        print!("{}, ", i);
    }
    println!();
    // ---------- break 式 ----------

    // ---------- continue 式 ----------
    let array = [2, 3, 0, 4, 5];
    for &i in &array {
        if i == 0 {
            continue;
        }
        print!("{}, ", i);
    }
    println!();
    // ---------- continue 式 ----------

    // ---------- .. operator ----------
    // a..b 「a 以上 b 未満」
    // a..=b 「a 以上 b 以下」
    for i in 0..5 {
        println!("{}", i);
    }
    // .. operator による無限ループ
    for i in 3.. {
        println!("{}", i);
        if i * i > 30 {
            break;
        }
    }
    // ---------- .. operator ----------

    // ---------- loop 式 ----------
    // loop 式では break; が実行されない限りブロックが繰り返し実行され続ける
    loop {
        proconio::input!(x: i32);
        if x > 0 {
            println!("{}", x * 2);
        } else {
            break;
        }
    }
    // 値を返す loop 式
    let value = loop {
        proconio::input!(x: i32);
        if x > 0 {
            println!("{}", x * 2);
        } else {
            // break の直後に式を書くと値を返せる
            break x;
        }
    };
    println!("value: {}", value);
    // ---------- loop 式 ----------

    // ---------- while 式 ----------
    let mut x = 15;
    let mut v = Vec::new();
    while x > 0 {
        v.push(x);
        x /= 2;
    }
    assert_eq!(x, 0);
    assert_eq!(v, vec![15, 7, 3, 1]);
    // ---------- while 式 ----------

    // ---------- ラベル ----------
    // 普通の2重ループ
    for i in 0..4 {
        for j in 0..i {
            print!("({}, {}) ", i, j);
        }
        println!();
    }

    // break を含む2重ループ
    for i in 0..4 {
        for j in 0..i {
            if i * j >= 2 {
                break;
            }
            print!("({}, {})", i, j);
        }
        println!();
    }

    // ラベルを使った2重ループ
    'outer: for i in 0..4 {
        for j in 0..i {
            if i * j >= 2 {
                break 'outer;
            }
            print!("({}, {}) ", i, j);
        }
        println!();
    }

    // 値を返すラベル付き loop 式
    let factor = 'input: loop {
        proconio::input!(x: i32);
        for i in 2.. {
            if i * i > x {
                break;
            } else if x % i == 0 {
                break 'input i;
            }
        }
    };
    println!("{}", factor);
    // ---------- ラベル ----------
}
