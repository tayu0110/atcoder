use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, x: [usize; n]}

    let mut map = BTreeSet::new();
    map.insert(0);
    let mut ret = 0;
    for x in x {
        match (map.range(..x).next_back(), map.range(x..).next()) {
            (Some(prev), Some(next)) => {
                ret += (x - prev).min(next - x);
                if let Some(prev_prev) = map.range(..*prev).next_back() {
                    ret -= (next - prev).min(prev - prev_prev);
                    ret += (x - prev).min(prev - prev_prev);
                } else {
                    ret -= next - prev;
                    ret += x - prev;
                }
                if let Some(next_next) = map.range(*next + 1..).next() {
                    ret -= (next_next - next).min(next - prev);
                    ret += (next_next - next).min(next - x);
                } else {
                    ret -= next - prev;
                    ret += next - x;
                }
            }
            (Some(prev), _) => {
                ret += x - prev;
                if let Some(prev_prev) = map.range(..*prev).next_back() {
                    ret -= prev - prev_prev;
                    ret += (x - prev).min(prev - prev_prev);
                } else {
                    ret += x - prev;
                }
            }
            (_, Some(next)) => {
                ret += next - x;
                if let Some(next_next) = map.range(*next + 1..).next() {
                    ret -= next_next - next;
                    ret += (next_next - next).min(next - x);
                } else {
                    ret += next - x;
                }
            }
            _ => {}
        }

        map.insert(x);
        println!("{ret}")
    }
}
