use fenwick_tree::{Addition, FenwickTree};
use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], q: usize, x: [usize; q]}

    if m == 1 {
        for _ in 0..q {
            println!("1");
        }
        return;
    }

    let mut count = vec![0; m + 1];
    for &a in &a {
        count[a] += 1;
    }
    let mut count = count
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<Vec<_>>();
    count.sort_unstable();

    let mut x = x
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<Vec<_>>();
    x.sort_unstable();

    let mut ft = FenwickTree::<Addition<usize>>::new(m + 1);
    let mut ret = vec![0; q];
    let mut base = 0;
    let mut to = n;
    let mut cur = 1;
    for (x, i) in x {
        if x <= n {
            ret[i] = a[x - 1];
            continue;
        }

        while to < x {
            let mut c = cur;
            while c < count.len() && count[c].0 == count[cur].0 {
                ft.add(count[c].1, 1);
                c += 1;
            }
            base = to + 1;
            if c == count.len() {
                to = usize::MAX;
            } else {
                let next = count[c].0;
                to += (next - count[cur].0) * (c - 1);
            }
            cur = c;
        }

        let rem = (x - base) % ft.fold(..);
        let r = ft.partition_point(|&t| t <= rem);
        ret[i] = r;
    }

    println!("{}", ret.into_iter().join("\n"))
}
