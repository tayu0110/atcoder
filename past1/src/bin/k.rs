use ds::LinkCutTree;
use proconio::*;

fn main() {
    input! {n: usize, p: [i32; n], q: usize}

    let mut lct = <LinkCutTree>::new(n);
    for (i, p) in p.into_iter().enumerate() {
        if p > 0 {
            lct.link(p as usize - 1, i).unwrap();
        }
    }

    for _ in 0..q {
        input! {a: usize, b: usize}

        if b - 1 == lct.lca(a - 1, b - 1).unwrap() {
            println!("Yes");
        } else {
            println!("No")
        }
    }
}
