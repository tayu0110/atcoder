use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: usize}

    let mut res = 0;
    for _ in (m..=n).step_by(p) {
        res += 1;
    }
    println!("{res}")
}
