use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut ret = 0;
    let mut pr = vec![0; n + 1];
    let mut pc = vec![0; n + 1];
    let mut cr = FenwickTree::<Addition<usize>>::new(q + 1);
    cr.add(0, n);
    let mut cc = FenwickTree::<Addition<usize>>::new(q + 1);
    cc.add(0, n);
    for i in 1..=q {
        input! {ty: u8, r: usize}

        if ty == 1 {
            cr.add(i, 1);
            cr.add(pr[r], !0);
        } else {
            cc.add(i, 1);
            cc.add(pc[r], !0);
        }

        if ty == 1 {
            if pr[r] == 0 {
                ret += n;
            } else {
                ret += cc.fold(pr[r] + 1..i);
            }
            pr[r] = i;
        } else {
            ret -= cr.fold(pc[r] + 1..i);
            pc[r] = i;
        }
        println!("{ret}")
    }
}
