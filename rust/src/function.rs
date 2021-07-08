fn main() {
    // 引数を取らない関数
    let vec = {
        let mut v = Vec::new();
        for i in 0..10 {
            v.push(i);
        }
        v
    };
    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    // 上に同じ
    fn digits() -> Vec<i32> {
        let mut v = Vec::new();
        for i in 0..10 {
            v.push(i);
        }
        v
    }
    let vec = digits();
    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // ユニット () を返す関数
    {
        println!("Hello");
    }
    // 上に同じ
    // 戻り値が () である場合には戻り値の宣言を省略できる
    fn greet() -> () {
        println!("Hello");
    }
    greet();

    // 関数のスコープは、ブロックの開始からブロックの終了まで
    {
        greet_1();
        fn greet_1() {
            println!("Hello");
        }
        greet_1();
    }

    // 環境 = 今存在している変数
    // スコープの概念は関数の中だけ
    // 関数が異なるとスコープ以前に環境が異なる
    {
        let a = 10;
        let b = 20;
        fn fnc() {
            let c = 30;
            // println!("{}", a); // ａ は fnc の環境の外
            println!("{}", c);
        }
    }
}
