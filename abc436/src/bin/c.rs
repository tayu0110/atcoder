use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {_: usize, m: usize, e: [(u32, u32); m]}

    let mut map = FxHashSet::default();

    for (r, c) in e {
        let l1 = (r, c);
        let l2 = (r, c + 1);
        let l3 = (r + 1, c);
        let l4 = (r + 1, c + 1);

        if !map.contains(&l1) && !map.contains(&l2) && !map.contains(&l3) && !map.contains(&l4) {
            map.insert(l1);
            map.insert(l2);
            map.insert(l3);
            map.insert(l4);
        }
    }

    println!("{}", map.len() / 4);
}
