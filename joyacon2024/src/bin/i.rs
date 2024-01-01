use proconio::*;

fn main() {
    input! {r: usize}

    if r < 1200 {
        println!("ABC")
    } else if r < 2800 {
        println!("ARC")
    } else {
        println!("AGC")
    }
}
