use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let sum = a.iter().sum::<usize>();
    let a = a.repeat(2);
    let (mut l, mut r) = (0, 0);
    while l < n {
        let mut s = 0;
        while r < a.len()
            && r - l < n - 1
            && (s + a[r]) * (n + l - r - 1) >= (sum - s - a[r]) * (r - l + 1)
        {
            s += a[r];
            r += 1;
        }

        if r - l == n - 1 {
            println!("{}", l + 1);
            return;
        }

        l = r.max(l + 1);
        r = l;
    }

    println!("-1");
}
