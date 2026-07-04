use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut a: [usize; n], b: [usize; q]}

    a.sort_unstable();
    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }
    let &max = a.iter().max().unwrap();
    let mut ans = vec![];
    for b in b {
        if b > max {
            ans.push(usize::MAX);
            continue;
        }

        let pos = a.partition_point(|&a| a < b);
        ans.push(cum[pos] + (b - 1) * (n - pos) + 1);
    }

    println!("{}", ans.iter().map(|&ans| ans as i64).join("\n"));
}
