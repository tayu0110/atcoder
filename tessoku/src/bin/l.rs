use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let (mut l, mut r) = (0, 1000_000_000_000);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut sum = 0;
        for &a in &a {
            sum += m / a;
        }

        if sum < k {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", r);
}
