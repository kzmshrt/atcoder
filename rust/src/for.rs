fn main() {
    practice_array_reference();
    practice_for();
}

///
/// # 配列の要素への参照
///
/// - `&primes[0]` は
///     - `&prime` に `[0]` が付いているのではない
///     - `prime[0]` に `&` が付いている
///     - `[]` 演算子は `&` 演算子よりも優先順位が高い
/// - 配列の各要素はメモリ上の連続した領域に配置される
/// - `&primes` は配列全体の先頭のアドレスだから `&primes[0]` と同じアドレス値である
///
fn practice_array_reference() {
    let primes = [2, 3, 5, 7];
    println!("{:p}", &primes[0]);
    println!("{:p}", &primes[1]);
    println!("{:p}", &primes[2]);
    println!("{:p}", &primes[3]);
    println!("{:p}", &primes);
}

///
/// # for (each) element in iterable
///
/// - 1 の for 式は 2 を実行することと同じ
/// - 内部的にはアドレス `&primes` に `4` を順次足していくことで書く要素の値が得られている
///
fn practice_for() {
    // 1
    let primes = [2, 3, 5, 7];
    for p in &primes {
        println!("{}", p);
    }

    // 2
    let primes = [2, 3, 5, 7];
    {
        let p = &primes[0];
        println!("{}", p);
    }
    {
        let p = &primes[1];
        println!("{}", p);
    }
    {
        let p = &primes[2];
        println!("{}", p);
    }
    {
        let p = &primes[3];
        println!("{}", p);
    }
}
