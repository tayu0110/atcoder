use proconio::*;

fn main() {
    input! {n: usize, mut p: [(i64, i64); n]}

    p.sort_unstable_by_key(|p| (p.0 - p.1, p.1));
    let mut res = 0;
    let mut maxb = 0;
    while let Some((a, b)) = p.pop() {
        res += a;
        maxb = maxb.max(b);
        println!("{}", res - maxb);
    }
}
