use proconio::input;

fn main() {
    input!(x: i32);
    if x < 10 {
        println!("10 未満");
    }
    println!("10 以上");

    if !(x > 0) {
        println!("x は正でない");
    }

    input!(ave_precip: f64, lowest_precip: f64, threshold: f64, lowest_temp: f64);
    if !(ave_precip < threshold) && lowest_temp >= 18. && (lowest_precip >= 60.) || lowest_precip >= 100. - 0.04 * ave_precip {
        println!("熱帯雨林気候（熱帯モンスーン気候を含む）である");
    } else {
        println!("熱帯雨林気候（熱帯モンスーン気候を含む）でない");
    }
}
