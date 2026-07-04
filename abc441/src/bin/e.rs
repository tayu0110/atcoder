use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut ft = FenwickTree::<Addition<usize>>::new(n * 2 + 10);
    ft.add(n, 1);
    let mut cum = vec![n];
    for (i, &b) in s.iter().enumerate() {
        match b {
            b'A' => cum.push(cum[i] + 1),
            b'B' => cum.push(cum[i] - 1),
            b'C' => cum.push(cum[i]),
            _ => {}
        }
        ft.add(cum[i + 1], 1);
    }

    let mut ret = 0;
    for c in cum {
        ft.add(c, 1usize.wrapping_neg());

        ret += ft.fold(c + 1..);
    }

    println!("{ret}")
}
