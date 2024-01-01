use proconio::*;

fn main() {
    input! {n: usize, s: usize, k: usize, p:[(usize, usize); n]}

    let mut res = 0;
    for (p, q) in p {
        res += p * q;
    }

    if res < s {
        res += k;
    }

    println!("{res}")
}
