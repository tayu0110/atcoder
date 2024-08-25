use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, s: usize}

    let l = n + m;
    let rem = l % k;
    for a in 0..=rem {
        if a > s {
            println!("No");
            return;
        }

        let b = s - a;
        if b > k - rem {
            continue;
        }
        if a * (l / k + 1) + b * (l / k) == m {
            let mut res = vec!['0'; l];
            for i in 0..a {
                for j in (i..l).step_by(k) {
                    res[j] = '1';
                }
            }
            for i in rem..rem + b {
                for j in (i..l).step_by(k) {
                    res[j] = '1';
                }
            }

            println!("Yes");
            println!("{}", res.iter().join(""));
            return;
        }
    }

    println!("No");
}
