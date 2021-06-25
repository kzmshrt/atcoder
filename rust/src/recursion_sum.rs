fn main() {
    sum_print(5);

    println!("{}", gcd(51, 15));
    println!("{}", gcd(15, 51));

    println!("{}", fibo(6));
    println!("{}", fibo_for(6));
    println!("{}", fibo_memo(6));
}

// 途中経過を出力しながら0からNまでの総和を求める
fn sum_print(n: i32) -> i32 {
    println!("Called sum_print({})", n);
    match n {
        0 => 0,
        _ => {
            let result = n + sum_print(n - 1);
            println!("Sum of 0 to {} = {}", n, result);
            result
        }
    }
}

// ユークリッドの互除法で最大公約数を求める
fn gcd(m: i32, n: i32) -> i32 {
    match n {
        0 => m,
        _ => gcd(n, m % n),
    }
}

// フィボナッチ数列の第N項を求める
fn fibo(n: i32) -> i32 {
    println!("Called fibo({})", n);
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let result = fibo(n - 1) + fibo(n - 2);
            println!("第{}項 = {}", n, result);
            result
        }
    }
}

// for ループでフィボナッチ数列の第N項を求める
fn fibo_for(n: i32) -> i32 {
    let len = n as usize;
    let mut fibo = vec![0; len + 1];
    fibo[0] = 0;
    fibo[1] = 1;
    for i in 2..=len {
        fibo[i] = fibo[i - 1] + fibo[i - 2];
        println!("第{}項: {}", i, fibo[i]);
    }
    fibo[len]
}

fn fibo_memo(n: i32) -> i32 {
    fn fibo_memo_inner(n: i32, memo: &mut Vec<i32>) -> i32 {
        let ns = n as usize;
        match n {
            0 => 0,
            1 => 1,
            _ => match memo[ns] {
                -1 => {
                    memo[ns] = fibo_memo_inner(n - 1, memo) + fibo_memo_inner(n - 2, memo);
                    memo[ns]
                }
                _ => memo[ns],
            },
        }
    }
    fibo_memo_inner(n, &mut vec![-1; (n + 1) as usize])
}
