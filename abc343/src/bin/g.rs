use proconio::*;
use string::RollingHash;

fn main() {
    input! {n: usize, mut s: [String; n]}

    let mut can_remove = vec![];
    for (i, u) in s.iter().enumerate() {
        if can_remove.contains(&i) {
            continue;
        }
        let ru = RollingHash::new(u.as_str());
        for (j, v) in s.iter().enumerate() {
            if i == j || can_remove.contains(&j) || u.len() < s.len() {
                continue;
            }

            let rv = RollingHash::new(v.as_str());

            for k in (0..u.len()).take_while(|k| k + v.len() <= u.len()) {
                if ru.get(k..k + v.len()) == rv.get(..) {
                    can_remove.push(j);
                    break;
                }
            }
        }
    }

    can_remove.sort_unstable();
    can_remove.dedup();
    while let Some(c) = can_remove.pop() {
        s.remove(c);
    }

    let n = s.len();
    let mut ben = vec![vec![0; n]; n];
    for (i, u) in s.iter().enumerate() {
        let ru = RollingHash::new(u.as_str());
        for (j, v) in s.iter().enumerate() {
            let rv = RollingHash::new(v.as_str());

            for k in 0..u.len() {
                if u.len() - k > v.len() {
                    continue;
                }

                if ru.get(k..) == rv.get(..u.len() - k) {
                    ben[i][j] = u.len() - k;
                    break;
                }
            }
        }
    }

    let mut dp = vec![vec![0; 1 << n]; n];
    for i in 0..1 << n {
        for j in 0..n {
            if i & (1 << j) == 0 {
                continue;
            }
            for k in 0..n {
                if i & (1 << k) != 0 {
                    continue;
                }

                let next = i | (1 << k);
                dp[k][next] = dp[k][next].max(dp[j][i] + ben[j][k]);
            }
        }
    }

    println!(
        "{}",
        s.iter().map(|s| s.len()).sum::<usize>()
            - dp.iter().map(|d| d[(1 << n) - 1]).max().unwrap()
    );
}
