use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut ans = vec![];
    for _ in 0..t {
        input! {n: usize, m: usize, mut a: [usize; n], mut b: [usize; n]}
        a.sort_unstable();
        b.sort_unstable();

        let mut res = a.iter().sum::<usize>() + b.iter().sum::<usize>();
        let mut now = 0;
        for &a in a.iter().rev() {
            while now < n && a + b[now] < m {
                now += 1;
            }

            if now < n {
                res -= m;
                now += 1;
            }
        }

        ans.push(res);
    }

    println!("{}", ans.iter().join("\n"))
}
