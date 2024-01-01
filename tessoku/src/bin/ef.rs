use fenwick_tree::FenwickTree;
use proconio::*;

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut res = 0u64;
    let mut ft = FenwickTree::new(n + 1, 0);
    for a in a {
        res += ft.get_sum(a as usize, n + 1) as u64;
        ft.add(a as usize, 1);
    }

    println!("{res}")
}
