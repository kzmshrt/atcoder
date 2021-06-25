fn count_753_num(k: i32) -> i32 {
    fn is_753_num(n: i32, c7: bool, c5: bool, c3: bool) -> bool {
        match n {
            0 => !c7 && !c5 && !c3,
            n => match n % 10 {
                7 => is_753_num(n / 10, false, c5, c3),
                5 => is_753_num(n / 10, c7, false, c3),
                3 => is_753_num(n / 10, c7, c5, false),
                _ => false,
            },
        }
    }
    let mut c = 0;
    for n in 0..=k {
        if is_753_num(n, true, true, true) {
            println!("{}", n);
            c += 1;
        }
    }
    c
}

fn main() {
    println!("{}", count_753_num(10000));
}
