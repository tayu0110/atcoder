use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, a: usize, mut s: marker::Chars}

    let mut set = s
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then_some(i + 1))
        .collect::<BTreeSet<_>>();
    let mut now = a;
    let mut right = true;
    let mut res = 0;
    let m = set.len();
    for _ in 0..m {
        if right {
            if let Some(&next) = set.range(now..).next() {
                set.remove(&next);
                res += next - now;
                now = next;
                right = false;
            } else {
                let last = set.pop_last().unwrap();
                res += n + 1 - now;
                res += n + 1 - last;
                set.remove(&last);
                now = last;
            }
        } else if let Some(&prev) = set.range(..now).next_back() {
            set.remove(&prev);
            res += now - prev;
            now = prev;
            right = true;
        } else {
            let first = set.pop_first().unwrap();
            res += now;
            res += first;
            now = first;
            set.remove(&first);
        }
    }

    println!("{res}")
}
