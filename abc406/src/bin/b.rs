use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut now = 1usize;
    for a in a {
        now = now.saturating_mul(a);

        if now.to_string().len() > k {
            now = 1;
        }
    }

    println!("{now}")
}
