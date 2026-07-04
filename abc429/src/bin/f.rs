use std::fmt::Write as _;

use ds::{Monoid, SegmentTree};
use proconio::*;

struct T;
impl Monoid for T {
    type M = [[u32; 3]; 3];
    fn id() -> Self::M {
        [[0; 3]; 3]
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        if *r == Self::id() {
            return *l;
        } else if *l == Self::id() {
            return *r;
        }
        let mut new = [[u32::MAX; 3]; 3];
        for from in 0..3 {
            for to in 0..3 {
                for via in 0..3 {
                    new[from][to] = new[from][to]
                        .min(l[from][via].saturating_add(r[via][to]).saturating_add(1));
                }
            }
        }
        new
    }
}

fn main() {
    input! {n: usize, mut s: [marker::Bytes; 3], q: usize}

    let make_matrix = |s: &[u8]| -> [[u32; 3]; 3] {
        match s {
            b"..." => [[0, 1, 2], [1, 0, 1], [2, 1, 0]],
            b"..#" => [[0, 1, u32::MAX], [1, 0, u32::MAX], [u32::MAX; 3]],
            b".#." => [
                [0, u32::MAX, u32::MAX],
                [u32::MAX; 3],
                [u32::MAX, u32::MAX, 0],
            ],
            b".##" => [[0, u32::MAX, u32::MAX], [u32::MAX; 3], [u32::MAX; 3]],
            b"#.." => [[u32::MAX; 3], [u32::MAX, 0, 1], [u32::MAX, 1, 0]],
            b"#.#" => [[u32::MAX; 3], [u32::MAX, 0, u32::MAX], [u32::MAX; 3]],
            b"##." => [[u32::MAX; 3], [u32::MAX; 3], [u32::MAX, u32::MAX, 0]],
            b"###" => [[u32::MAX; 3]; 3],
            _ => unreachable!(),
        }
    };
    let mut st = SegmentTree::<T>::new(n);
    for i in 0..n {
        let mat = make_matrix(&[s[0][i], s[1][i], s[2][i]]);
        st.set(i, mat);
    }

    let mut buf = String::new();
    for _ in 0..q {
        input! {r: usize, c: usize}
        s[r - 1][c - 1] ^= b'#' ^ b'.';
        st.set(c - 1, make_matrix(&[s[0][c - 1], s[1][c - 1], s[2][c - 1]]));

        let folded = st.fold(..);
        writeln!(buf, "{}", folded[0][2] as i32).unwrap();
    }

    print!("{buf}")
}
