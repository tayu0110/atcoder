use itertools::Itertools;
use proconio::*;
use utility::BinaryGrid;

fn main() {
    input! {n: usize, s: [marker::Chars; n]}

    let grid = BinaryGrid::<'o', 'x'>::from_vec(s).unwrap();
    let res = grid.rotate90().rotate180().restore();

    for s in res {
        println!("{}", s.iter().join(""))
    }
}
