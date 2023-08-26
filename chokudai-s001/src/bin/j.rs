use fenwick_tree::FenwickTree;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0usize;
    let mut ft = FenwickTree::new(n + 1, 0);
    for a in a {
        res += ft.get_sum(a, n + 1);
        ft.add(a, 1);
    }

    println!("{}", res)
}
