use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut min = std::usize::MAX;
    for i in 0usize..1 << n {
        let mut b = i & 1;
        let mut now = 0;
        let mut res = 0;
        for j in 0..n {
            if (i >> j) & 1 ^ b != 0 {
                res ^= now;
                now = a[j];
                b ^= 1;
            } else {
                now |= a[j];
            }
        }
        res ^= now;

        min = min.min(res);
    }

    println!("{}", min)
}
