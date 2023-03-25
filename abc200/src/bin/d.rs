use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    let a = a.into_iter().map(|c| c % 200).collect::<Vec<_>>();

    let mut dp = vec![vec![]; 200];
    dp[0].push(vec![]);

    for (i, a) in a.into_iter().enumerate() {
        let mut tmp = vec![vec![]; 200];
        for j in 0..200 {
            for v in &dp[j] {
                let next = (j + a) % 200;
                let mut now = v.clone();
                now.push(i + 1);
                tmp[next].push(now);

                if tmp[next].len() >= 2 {
                    println!("Yes");
                    println!("{} {}", tmp[next][0].len(), tmp[next][0].iter().join(" "));
                    println!("{} {}", tmp[next][1].len(), tmp[next][1].iter().join(" "));
                    return;
                }
            }
        }

        for j in 0..200 {
            if tmp[j].is_empty() {
                continue;
            }
            dp[j].push(tmp[j][0].clone());

            if j > 0 && dp[j].len() >= 2 {
                println!("Yes");
                println!("{} {}", dp[j][0].len(), dp[j][0].iter().join(" "));
                println!("{} {}", dp[j][1].len(), dp[j][1].iter().join(" "));
                return;
            } else if j == 0 && dp[j].len() >= 3 {
                println!("Yes");
                println!("{} {}", dp[j][1].len(), dp[j][1].iter().join(" "));
                println!("{} {}", dp[j][2].len(), dp[j][2].iter().join(" "));
                return;
            }
        }
    }

    println!("No");
}
