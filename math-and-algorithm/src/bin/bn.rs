use proconio::*;

fn main() {
    input! {n: usize, mut p: [(u32, u32); n]}
    p.sort_unstable_by_key(|&(l, r)| (r, l));

    let mut now = 0;
    let mut cnt = 0;
    for (l, r) in p {
        if now <= l {
            now = r;
            cnt += 1;
        }
    }

    println!("{cnt}")
}
