use ds::BTreeMultiSet;
use proconio::*;

fn main() {
    input! {q: usize}

    let mut set = BTreeMultiSet::new();
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: usize}
            set.insert(x);
        } else if ty == 2 {
            input! {x: usize, k: usize}
            let t = set.range(..=x).rev().take(k).collect::<Vec<_>>();
            if t.len() < k {
                println!("-1");
            } else {
                println!("{}", t.last().unwrap());
            }
        } else {
            input! {x: usize, k: usize}
            let t = set.range(x..).take(k).collect::<Vec<_>>();
            if t.len() < k {
                println!("-1");
            } else {
                println!("{}", t.last().unwrap());
            }
        }
    }
}
