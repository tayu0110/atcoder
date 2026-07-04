use proconio::*;

fn main() {
    input! {s: usize, a: usize, b: usize, mut x: usize}

    let mut ret = 0;
    while x > 0 {
        ret += s * x.min(a);
        x = x.saturating_sub(a + b);
    }

    println!("{ret}")
}
