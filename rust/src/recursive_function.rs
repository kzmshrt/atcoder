fn main() {
    for i in 0..=10 {
        println!("{}", fact(i));
    }

    assert_eq!(sum(&[5, -1, 3, 4, -2]), 9);

    let mut v = vec![1, 4, 14, 1, 4, 1, 4, 13923, 9013, 934, 56];
    let s = &mut v;
    quick_sort(s);
    println!("{:?}", s);
}

fn fact(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        fact(n - 1) * n
    }
}

fn sum(slice: &[i32]) -> i32 {
    if slice.is_empty() {
        0
    } else {
        sum(&slice[1..]) + slice[0]
    }
}

fn quick_sort(slice: &mut [i32]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    let pivot = slice[0];
    let mut left = 1;
    let mut right = len - 1;
    loop {
        while left < len && slice[left] <= pivot {
            left += 1;
        }
        while 0 < right && pivot < slice[right] {
            right -= 1;
        }
        if left == right + 1 {
            slice.swap(0, right);
            break;
        } else if left < right {
            slice.swap(left, right);
            left += 1;
            right -= 1;
        } else {
            panic!();
        }
    }
    quick_sort(&mut slice[..right]);
    quick_sort(&mut slice[left..]);
}
