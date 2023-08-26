use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let mut res = 1;
    for _ in 0..n {
        res = (res * 2).min(res + k);
    }

    println!("{}", res)
}
