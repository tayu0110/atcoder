use proconio::*;

fn main() {
    input! {m: usize, n: usize, k: usize, x: [usize; n]}

    let mut res = 0;
    for i in 1..=m {
        let mut t = vec![0; k + 1];
        for &x in &x {
            let diff = x.max(i) - x.min(i);
            if diff <= k {
                if diff == 0 {
                    t[diff] += 1;
                } else {
                    t[diff] = 1;
                }
            }
        }

        res = res.max(t.into_iter().sum())
    }

    println!("{}", res)
}
