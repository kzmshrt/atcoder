fn main() {
    practice_arithmetic_operator();
    practice_partial_eq();
    practice_function_returns_bool();
    practice_logical_operator();
}

fn practice_arithmetic_operator() {
    assert_eq!(std::ops::Add::add(2, 3), 2 + 3);
    assert_eq!(std::ops::Sub::sub(5, 2), 5 - 2);
    assert_eq!(std::ops::Mul::mul(3, 4), 3 * 4);
    assert_eq!(std::ops::Div::div(14, 3), 14 / 3);
    assert_eq!(std::ops::Rem::rem(14, 3), 14 % 3);
}

fn practice_logical_operator() {
    // AND &
    assert_eq!(true & true, true);
    assert_eq!(true & false, false);
    assert_eq!(false & true, false);
    assert_eq!(false & false, false);

    fn fnc1() -> bool {
        println!("fnc1: false");
        false
    }
    fn fnc2() -> bool {
        println!("fnc2: true");
        true
    }

    let result = fnc1() & fnc2();
    assert_eq!(result, false);

    let result = fnc1() && fnc2();
    assert_eq!(result, false);

    // OR |
    assert_eq!(true | true, true);
    assert_eq!(true | false, true);
    assert_eq!(false | true, true);
    assert_eq!(false | false, false);

    let result = fnc2() | fnc1();
    assert_eq!(result, true);

    let result = fnc2() || fnc1();
    assert_eq!(result, true);

    // XOR ^
    assert_eq!(true ^ true, false);
    assert_eq!(true ^ false, true);
    assert_eq!(false ^ true, true);
    assert_eq!(false ^ false, false);

    // Not !
    assert_eq!(!true, false);
    assert_eq!(!false, true);

    // 複合代入演算子
    let mut flag = true;
    flag &= false;
    assert_eq!(flag, false);
    flag |= false;
    assert_eq!(flag, false);
    flag |= true;
    assert_eq!(flag, true);
    flag ^= true;
    assert_eq!(flag, false);
}

fn practice_partial_eq() {
    let x: i32 = 5;
    if PartialEq::eq(&x, &5) {
        println!("x is equal to 5.");
    }
}

fn practice_function_returns_bool() {
    fn is_prime(x: i32) -> bool {
        if x < 2 {
            return false;
        }
        for i in 2.. {
            if i * i > x {
                return true;
            }
            if x % i == 0 {
                return false;
            }
        }
        unreachable!();
    }

    for i in 0..=100 {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}
