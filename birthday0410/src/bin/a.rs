use proconio::*;

fn main() {
    input! {t: usize}

    if t == 1 {
        println!("-1 1");
    } else if t == 2 {
        println!("{} {}", std::i32::MIN, std::i32::MAX);
    } else {
        println!("{} {}", -2, 2);
    }
    println!("0 0");
}
