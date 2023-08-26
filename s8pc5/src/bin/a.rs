use proconio::*;

fn main() {
    input! {n: usize, t: usize, a: [usize; n]}

    let mut now = 0;
    for a in a {
        if now < a {
            now = a;
        }
        let rem = now - a;
        now = a + (rem + t - 1) / t * t;
    }

    println!("{}", now)
}
