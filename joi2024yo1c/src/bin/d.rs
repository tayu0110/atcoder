use proconio::*;

fn main() {
    input! {k: usize, n: usize, a: [usize; n], m: usize, b: [usize; m]}

    let mut res = 0;
    for a in a {
        for &b in &b {
            if a + k == b {
                res += 1;
            }
        }
    }

    println!("{res}")
}
