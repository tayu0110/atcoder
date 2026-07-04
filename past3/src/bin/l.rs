use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize}

    let mut f = BTreeSet::new();
    let mut b = BTreeSet::new();
    let mut bucket = vec![vec![]; n];
    let mut t = vec![];
    for i in 0..n {
        input! {k: usize, nt: [usize; k]}
        f.insert((nt[0], i, 0));
        bucket[i].push(0);
        if nt.len() >= 2 {
            b.insert((nt[1], i, 1));
            bucket[i].push(1);
        }
        t.push(nt);
    }

    input! {m: usize, a: [usize; m]}

    for a in a {
        if a == 1 || b.is_empty() {
            let (res, i, _) = f.pop_last().unwrap();
            println!("{res}");

            bucket[i].remove(0);
            if !bucket[i].is_empty() {
                b.remove(&(t[i][bucket[i][0]], i, bucket[i][0]));
                f.insert((t[i][bucket[i][0]], i, bucket[i][0]));
                if bucket[i][0] + 1 < t[i].len() {
                    let new = bucket[i][0] + 1;
                    bucket[i].push(new);
                    b.insert((t[i][new], i, new));
                }
            }
        } else {
            let &(fres, fi, fpos) = f.last().unwrap();
            let &(bres, bi, bpos) = b.last().unwrap();
            if fres > bres {
                println!("{fres}");
                f.remove(&(fres, fi, fpos));
                bucket[fi].remove(0);
                if !bucket[fi].is_empty() {
                    b.remove(&(t[fi][bucket[fi][0]], fi, bucket[fi][0]));
                    f.insert((t[fi][bucket[fi][0]], fi, bucket[fi][0]));
                    if bucket[fi][0] + 1 < t[fi].len() {
                        let new = bucket[fi][0] + 1;
                        bucket[fi].push(new);
                        b.insert((t[fi][new], fi, new));
                    }
                }
            } else {
                println!("{bres}");
                b.remove(&(bres, bi, bpos));
                bucket[bi].pop();
                if bpos + 1 < t[bi].len() {
                    bucket[bi].push(bpos + 1);
                    b.insert((t[bi][bpos + 1], bi, bpos + 1));
                }
            }
        }
    }
}
