fn main() {
    practice_vector_moved_to_function();
    practice_pass_by_reference();
}

fn practice_vector_moved_to_function() {
    fn sum(v: Vec<i32>) -> i32 {
        let mut ret = 0;
        for &i in &v {
            ret += i;
        }
        ret
    }

    // vector はベクタの所有権を持つ
    let vector = vec![20, 80, 60, 40];

    // sum の開始時点で vector が持っていたベクタへの所有権は v へ移動（ムーブ）し、sum の終了時点でベクタの所有権を持つ v のスコープが終了し、ベクタはその時点でドロップされる
    let s = sum(vector);

    // vector は所有権を失ったままになる
    assert_eq!(s, 200);

    // sum(vector) 以降 vector を使用することはできなくなる
    // println!("{:?}", vector);
}

fn practice_pass_by_reference() {
    fn sum(v: &Vec<i32>) -> i32 {
        let mut ret = 0;
        for &i in v {
            ret += i;
        }
        ret
    }

    let vector = vec![20, 80, 60, 40];
    let s = sum(&vector);
    println!("sum of {:?}: {}", vector, s);
}
