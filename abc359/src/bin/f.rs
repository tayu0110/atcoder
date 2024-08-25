use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    if n == 2 {
        println!("{}", a[0] + a[1]);
        return;
    }

    a.sort_unstable();
    let mut res = 0;
    res += a.pop().unwrap();
    res += a.pop().unwrap();

    let mut nt = a
        .iter()
        .enumerate()
        .map(|(i, &a)| Reverse((5 * a, 2, a, i)))
        .collect::<BinaryHeap<_>>();
    let mut max = a
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i))
        .collect::<BinaryHeap<_>>();

    let mut used = vec![false; n];
    let mut removed = vec![false; n];
    while let Some((a, i)) = max.pop() {
        if used[i] {
            continue;
        }

        let Reverse((inc, d, pa, j)) = nt.pop().unwrap();
        if inc >= 4 * a - a {
            nt.push(Reverse((inc, d, pa, j)));
            max.push((a, i));
            break;
        }

        res += a;
        let nd = d + 1;
        nt.push(Reverse((((nd + 1) * (nd + 1) - nd * nd) * pa, nd, pa, j)));
        used[i] = true;
        used[j] = true;
        removed[i] = true;
    }

    // eprintln!("nt: {nt:?}");
    res += nt
        .into_iter()
        .filter(|Reverse(v)| !removed[v.3])
        .map(|Reverse(v)| v.1 * v.1 * v.2)
        .sum::<usize>();
    println!("{res}")
}
