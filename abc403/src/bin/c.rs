use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, _m: usize, q: usize}

    let mut ok = vec![FxHashSet::default(); n + 1];
    let mut all = vec![false; n + 1];
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize, y: usize}
            ok[x].insert(y);
        } else if ty == 2 {
            input! {x: usize}
            all[x] = true;
        } else {
            input! {x: usize, y: usize}
            if all[x] || ok[x].contains(&y) {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}
