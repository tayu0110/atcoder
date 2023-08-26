use proconio::*;

fn main() {
    input! {t: usize}
    eprintln!("{}", std::i64::MIN);
    for _ in 0..t {
        input! {a: i64, b: i64}
        let (t, f) = a.overflowing_div(b);
        if f {
            println!("{}", a as i128 / b as i128)
        } else {
            println!("{}", t)
        }
    }
}
