use proconio::*;

fn main() {
    input! {n: usize, m: usize, t: [usize; n]}

    let (mut l, mut r) = (0, 1_000_000_000_000_000);
    while r - l > 1 {
        let w = (r + l) / 2;
        let mut lines = 0;
        let mut now = w;
        for &t in &t {
            if t > w {
                lines = usize::MAX;
                break;
            }

            if now + t > w {
                lines += 1;
                now = 0;
            }

            now += t + 1;
        }

        if lines <= m {
            r = w;
        } else {
            l = w;
        }
    }

    println!("{r}")
}
