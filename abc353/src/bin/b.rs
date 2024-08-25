use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut res = 0;
    let mut rem = k;
    for a in a {
        if rem < a {
            res += 1;
            rem = k - a;
        } else {
            rem -= a;
        }
    }

    if rem < k {
        res += 1;
    }

    println!("{res}");
}
