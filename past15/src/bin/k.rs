use fenwick_tree::FenwickTree;
use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut front = FenwickTree::<i64>::new(n + 1, 0);
    let mut back = FenwickTree::<i64>::new(n + 1, 0);
    for i in 1..=n {
        back.add(i, 1);
    }

    let mut res = 0;
    for p in p {
        back.add(p, -1);
        res += p as i64 * (front.get_sum(p, n + 1) + back.get_sum(0, p));
        front.add(p, 1);
    }

    println!("{res}")
}
