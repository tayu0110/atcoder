use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n]}

    let mut res = 0;
    let mut now = 0;
    while now < n {
        let mut k = now;
        while k + 1 < n && h[k] >= h[k + 1] {
            k += 1;
        }

        res = res.max(k - now);
        now = k + 1;
    }

    println!("{}", res)
}
