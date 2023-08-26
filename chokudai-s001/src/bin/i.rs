use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    let mut sum = 0;
    let (mut l, mut r) = (0, 0);
    while l < n {
        while r < n && sum + a[r] <= n {
            sum += a[r];
            if sum == n {
                res += 1;
            }
            r += 1;
        }

        sum -= a[l];
        l += 1;
    }

    println!("{}", res)
}
