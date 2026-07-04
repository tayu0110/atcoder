use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], mut b: [usize; m]}
    b.sort_unstable();

    let mut cum = vec![0; m + 1];
    for i in 0..m {
        cum[i + 1] = cum[i] + b[i];
    }

    let mut ret = 0;
    for a in a {
        let pos = b.partition_point(|&b| b < a);
        ret += a * pos - cum[pos];
        ret += cum[m] - cum[pos] - a * (m - pos);
        ret %= M;
    }

    println!("{ret}")
}
