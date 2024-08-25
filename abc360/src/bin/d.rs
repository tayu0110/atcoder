use proconio::*;

fn main() {
    input! {n: usize, t: i64, s: marker::Bytes, x: [i64; n]}

    let (u, mut v) = x
        .iter()
        .cloned()
        .enumerate()
        .map(|x| (x.1, x.0))
        .partition::<Vec<_>, _>(|&(_, i)| s[i] < b'1');

    v.sort_unstable();

    let mut res = 0;
    for (x, _) in u {
        let prev = x - 2 * t;
        res += v.partition_point(|&v| v.0 <= x) - v.partition_point(|&v| v.0 < prev);
    }

    println!("{res}")
}
