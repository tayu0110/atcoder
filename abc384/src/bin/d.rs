use proconio::*;

fn main() {
    input! {n: usize, mut s: usize, a: [usize; n]}

    let sum = a.iter().sum::<usize>();
    s %= sum;

    let a = a.repeat(2);
    let n = a.len();
    let (mut l, mut r) = (0, 0);
    let mut sum = 0;
    while l < n {
        while r < n && sum + a[r] <= s {
            sum += a[r];
            r += 1;
        }

        if s == sum {
            println!("Yes");
            return;
        }

        if l < r {
            sum -= a[l];
            l += 1;
        } else {
            l += 1;
            r += 1;
        }
    }

    println!("No")
}
