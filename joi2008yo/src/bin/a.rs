use proconio::*;

fn main() {
    input! {n: usize}

    let mut m = 1000 - n;
    let mut res = 0;
    for c in [500, 100, 50, 10, 5, 1] {
        res += m / c;
        m %= c;
    }
    println!("{res}")
}
