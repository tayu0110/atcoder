use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n]}
    a.sort_unstable();

    let mut res = 0;
    let (mut l, mut r) = (0, 0);
    while l < n {
        while r < n && a[r] - a[l] < m {
            r += 1;
        }

        res = res.max(r - l);

        l += 1;
    }

    println!("{res}")
}
