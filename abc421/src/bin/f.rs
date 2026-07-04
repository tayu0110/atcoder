use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {q: usize}

    let mut prev = vec![usize::MAX; q + 1];
    let mut next = vec![usize::MAX; q + 1];
    let mut remove = vec![];
    for i in 1..=q {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize}

            let old_next = next[x];
            next[x] = i;
            next[i] = old_next;

            if old_next < usize::MAX {
                let old_prev = prev[old_next];
                prev[old_prev] = i;
            }
            prev[i] = x;
        } else {
            input! {x: usize, y: usize}
            remove.push((x, y, i));
        }
    }

    let mut arr = vec![];
    let mut rev = vec![0; q + 1];
    let mut now = 0;
    for i in 0..q + 1 {
        arr.push(now);
        rev[now] = i;
        now = next[now];
        if now == usize::MAX {
            break;
        }
    }

    let mut memo_start = vec![vec![]; q + 1];
    let mut memo_end = vec![vec![]; q + 1];
    for &(x, y, i) in &remove {
        let posx = rev[x];
        let posy = rev[y];
        if posx < posy {
            memo_start[x].push(i);
            memo_end[y].push(i);
        } else {
            memo_start[y].push(i);
            memo_end[x].push(i);
        }
    }

    let mut memo = BTreeSet::new();
    let remove = remove.into_iter().map(|v| v.2).collect::<HashSet<_>>();
    let mut res = vec![0; q + 1];
    for now in arr {
        for &end in &memo_end[now] {
            memo.remove(&end);
        }
        if let Some(&min) = memo.range(now..).next() {
            res[min] += now;
        }
        for &start in &memo_start[now] {
            memo.insert(start);
        }
    }

    println!(
        "{}",
        res.into_iter()
            .enumerate()
            .filter_map(|(i, res)| remove.contains(&i).then_some(res))
            .join("\n")
    )
}
