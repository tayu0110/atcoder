use modint::{combination, Mod998244353, StaticModint};
use proconio::input;

type Mint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let set = a.into_iter().collect::<std::collections::HashSet<_>>();
    let n = set.len();

    let mut dist = vec![0; n + k + 1];
    for i in 0..n + k {
        dist[i + 1] = dist[i];
        if !set.contains(&i) {
            dist[i + 1] += 1;
        }
    }

    let mut res = Mint::zero();
    let com = combination((n + k) as u32);
    for i in 0..=n + k {
        if dist[i] > k {
            continue;
        }

        res += com(k - dist[i] + i - 1, i);
    }

    println!("{}", res);
}
