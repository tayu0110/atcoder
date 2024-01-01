use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut res = 0;
    let mut sum = 0;
    let (mut l, mut r) = (0, 0);
    while l < n {
        while r < n && sum + a[r] <= k {
            sum += a[r];
            r += 1;
        }

        res += r - l;
        sum -= a[l];
        l += 1;
    }

    println!("{res}")
}
