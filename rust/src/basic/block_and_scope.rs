fn main() {}

fn block_order() {
    println!("1");
    {
        println!("2");
        println!("3");
    }
    println!("4")
}

fn block_scope() {
    // ブロックの内側で宣言された変数を、ブロックの外側で使用することはできない
    // {
    //     let hoge = 10;
    // }
    // println!("{}", hoge);

    // ブロックの外側で宣言された変数を、ブロックの内側で使用することはできる
    let hoge = 10;
    {
        println!("{}", hoge);
    }
}

fn block_shadowing() {
    // 同じ名前の変数を複数回宣言できる
    let hoge = 10;
    println!("{}", hoge);
    let hoge = 20;
    println!("{}", hoge);

    // シャドーイングにブロックを挟む
    let hoge = 10;
    {
        println!("{}", hoge); // 10
        let hoge = 20;
        println!("{}", hoge); // 20
    }
    println!("{}", hoge) // 10

    // シャドーイングを行う場合、同じ名前でも全く別の変数として宣言されるので、型が異なっても構わない
    let hoge: i32 = 10;
    println!("{}", hoge);
    let hoge: f64 = 10.052;
    println!("{}", hoge);
}

fn block_return_value() {
    println!("before block");
    let hoge = {
        println!("in block");
        10
    };
    println!("after block, hoge = {}", hoge);
}

fn block_if_expression() {
    let abs;
    abs = if x >= 0 { x } else { -x };
    println!("絶対値 = {}", abs);
}
