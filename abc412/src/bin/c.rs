use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut ans = vec![];
    for _ in 0..t {
        input! {n: usize, mut s: [usize; n]}

        if n == 2 {
            if s[0] * 2 >= s[1] {
                ans.push(2);
            } else {
                ans.push(-1);
            }
            continue;
        }
        s[1..n - 1].sort_unstable();
        let mut now = s[0];
        let mut res = 0;
        let mut i = 1;
        while now * 2 < s[n - 1] && i < n - 1 {
            let pos = s[i..n - 1].partition_point(|&s| now * 2 >= s) + i;
            if i < pos && now * 2 >= s[pos - 1] {
                i = pos;
                now = s[pos - 1];
                res += 1;
            } else {
                break;
            }
        }

        if now * 2 < s[n - 1] {
            ans.push(-1);
        } else {
            ans.push(res as i32 + 2);
        }
    }

    println!("{}", ans.into_iter().join("\n"));
}
