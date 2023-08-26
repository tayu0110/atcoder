// use proconio::*;
use rand::{thread_rng, Rng};
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn naive(p: &[(usize, usize)], a: &[usize]) -> Modint {
    if a.len() == 2 && p.contains(&(a[0], a[1])) {
        return Modint::one();
    }

    let mut res = Modint::zero();
    for (i, v) in a.windows(2).enumerate() {
        if p.contains(&(v[0], v[1])) {
            let new = if i + 2 < a.len() {
                [&a[..i], &a[i + 2..]].concat()
            } else {
                a[..i].to_vec()
            };
            res += naive(p, &new);
        }
    }
    res
}

fn main() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(2, 4);
    let m: usize = rng.gen_range(n, n * (2 * n - 1));
    let p = {
        let mut buf = vec![];
        while buf.len() < m {
            for _ in buf.len()..m {
                let a = rng.gen_range(1, 2 * n);
                let b = rng.gen_range(a + 1, 2 * n + 1);
                buf.push((a, b));
            }
            buf.sort();
            buf.dedup();
        }
        buf
    };
    eprintln!("n: {}, m: {}, p: {:?}", n, m, p);
    // input! {n: usize, m: usize, p: [(usize, usize); m]}
    let m = 2 * n;

    let com = combination::<Mod998244353>(m as u32 + 10);

    let a = (1..=m).collect::<Vec<_>>();
    let naive_res = naive(&p, &a);

    let mut dp = vec![vec![Modint::zero(); m]; m];
    for (a, b) in p {
        dp[a - 1][b - 1] = Modint::one();
    }
    for len in (2..=m).step_by(2) {
        for l in 0..m {
            let r = l + len - 1;
            if r >= m {
                break;
            }

            for mid in l + 1..r {
                let new = dp[l][mid] * dp[mid + 1][r] * com(len / 2, (r - mid) / 2);
                dp[l][r] += new;
            }
        }
    }

    eprintln!("dp: {:?}", dp);

    eprintln!("naive: {}, clever: {}", naive_res, dp[0][m - 1]);
    assert_eq!(naive_res, dp[0][m - 1]);

    println!("{}", dp[0][m - 1])
}
