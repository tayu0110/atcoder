use proconio::*;

fn main() {
    input! {n: usize, c: [usize; n], x: [usize; n]}

    let c = c.repeat(2);
    let mut memo = vec![vec![usize::MAX; 2 * n + 1]; 2 * n];
}
