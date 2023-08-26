use itertools::Itertools;
use proconio::*;

const MAX: usize = 10001;

fn main() {
    input! {n: usize, x: usize, a: [usize; n]}

    let mut res = 0;
    let mut dp = vec![std::i32::MIN; MAX];
    dp[x] = 0;
    for a in a {
        let mut new = dp.clone();
        for i in 0..MAX {
            if dp[i] == std::i32::MIN {
                continue;
            }
            let mut rem = i.to_string().chars().collect::<Vec<_>>();
            rem.sort();
            for v in rem.iter().permutations(rem.len()) {
                let to = v
                    .into_iter()
                    .fold(0, |s, &v| s * 10 + (v as usize - b'0' as usize));
                if to < a {
                    continue;
                }
                new[to - a] = new[to - a].max(dp[i] + 1);
            }
        }

        dp = new;
        res = res.max(*dp.iter().max().unwrap())
    }

    println!("{}", res)
}
