use proconio::*;

fn main() {
    input! {n: usize, p: usize, s: marker::Chars}
    let s = s
        .into_iter()
        .map(|v| (v as usize - b'0' as usize))
        .collect::<Vec<_>>();

    let mut np = vec![0; n];
    let mut ten = 1;
    np[n - 1] = s[n - 1] % p;
    for i in (0..n - 1).rev() {
        ten *= 10;
        ten %= p;

        np[i] = s[i] * ten + np[i + 1];
        np[i] %= p;
    }

    let mut memo = vec![0; p];
    for i in 0..n {
        memo[np[i]] += 1;
    }

    let mut res = 0usize;
    for i in (0..n).rev() {
        if np[i] == 0 {
            res += memo[np[i]];
            memo[np[i]] -= 1;
        } else {
            memo[np[i]] -= 1;
            res += memo[np[i]];
        }
    }

    println!("{}", res)
}
