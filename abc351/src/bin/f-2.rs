use ds::StaticRectangleSum;
use proconio::*;

fn main() {
    input! {n: usize, a: [u64; n]}

    let wm = a
        .iter()
        .map(|&a| (a, a))
        .collect::<StaticRectangleSum<_, _>>();
    let mut res = 0;
    for (i, a) in a.into_iter().enumerate() {
        res += wm.sum_of_weight(a.., i + 1..);
        res -= a * wm.count_within(a.., i + 1..) as u64;
    }

    println!("{res}");
}
