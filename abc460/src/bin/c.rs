use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], mut b: [usize; m]}

    a.sort_unstable();
    b.sort_unstable();

    let mut ret = 0;
    let mut now = 0;
    for &a in &a {
        if now < m && b[now] <= a * 2 {
            ret += 1;
            now += 1;
        }
    }

    println!("{ret}")
}
