fn tribonacci(n: i64) -> i64 {
    fn tribo(n: i64, m: &mut Vec<i64>) -> i64 {
        let l = n as usize;
        match n {
            0 => 0,
            1 => 0,
            2 => 1,
            _ => match m[l] {
                -1 => {
                    m[l] = tribo(n - 1, m) + tribo(n - 2, m) + tribo(n - 3, m);
                    m[l]
                }
                _ => m[l],
            },
        }
    }
    tribo(n, &mut vec![-1; (n + 1) as usize])
}

fn main() {
    for i in 0..50 {
        println!("{}", tribonacci(i));
    }
}
