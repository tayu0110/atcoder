use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let sum = a.iter().sum::<usize>();

    let mut b = a.clone();
    b.extend(a);

    let (mut l, mut r) = (0, 0);
    let mut now = 0;
    let mut res = sum;
    while l < b.len() {
        while r - l < n && r < b.len() && 2 * now < sum {
            now += b[r];
            res = res.min(now.abs_diff(sum - now));
            r += 1;
        }

        if l < r {
            now -= b[l];
            l += 1;
        }

        if l > r {
            r += 1;
        }
    }

    println!("{res}")
}
