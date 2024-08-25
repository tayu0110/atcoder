use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, mut d: [usize; n]}
    d.iter_mut().for_each(|d| *d %= a + b);
    d.sort_unstable();

    let mut d = d.repeat(2);
    d[n..].iter_mut().for_each(|d| *d += a + b);

    let (mut l, mut r) = (0, 0);
    while l < n {
        while r < d.len() && d[r] - d[l] < a {
            r += 1;
        }

        if r - l >= n {
            println!("Yes");
            return;
        }

        l += 1;
        if r < l {
            r += 1;
        }
    }

    println!("No");
}
