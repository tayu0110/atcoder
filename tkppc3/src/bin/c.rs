use proconio::*;

fn main() {
    input! {n: usize, p: usize, a: [usize; n]}

    let (mut l, mut r) = (0, 0);
    let mut now = 1;
    while l < n {
        while r < n && now * a[r] <= p {
            now *= a[r];
            r += 1;
        }

        if now == p {
            println!("Yay!");
            return;
        }

        now /= a[l];
        l += 1;
    }

    println!(":(")
}
