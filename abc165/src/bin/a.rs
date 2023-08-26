use proconio::*;

fn main() {
    input! {k: usize, a: usize, b: usize}

    for i in a..=b {
        if i % k == 0 {
            println!("OK");
            return;
        }
    }
    println!("NG")
}
