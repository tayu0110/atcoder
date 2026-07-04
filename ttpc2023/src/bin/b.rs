use std::cmp::Ordering;

use proconio::*;

fn main() {
    input! {n: usize, s: [usize; n]}

    let mut t = vec![];
    for mut s in s {
        let mut buf = vec![];
        while s > 0 {
            buf.push(s % 3);
            s /= 3;
        }
        buf.resize(12, 0);
        t.push(buf);
    }

    let mut index = (0..n).collect::<Vec<_>>();
    let mut ok = vec![false; n];
    ok[0] = true;

    for i in 0..12 {
        index.sort_unstable_by(|&l, &r| {
            if t[l]
                .iter()
                .zip(t[r].iter())
                .enumerate()
                .filter(|(k, _)| *k != i)
                .all(|(_, (l, r))| l < r)
            {
                Ordering::Less
            } else if t[l]
                .iter()
                .zip(t[r].iter())
                .enumerate()
                .filter(|(k, _)| *k != i)
                .all(|(_, (l, r))| l > r)
            {
                Ordering::Greater
            } else {
                ok[l].cmp(&ok[r])
            }
        });

        let pos = ok.iter().position(|i| *i).unwrap();
        index[pos..].iter().for_each(|&i| ok[i] = true);
    }

    if ok[n - 1] {
        println!("Yes")
    } else {
        println!("No")
    }
}
