use std::cmp::Reverse;

use cpio::*;

fn main() {
    scan!(n: usize, q: usize, mut b: [(u32, u32); n], t: [u32; q]);

    b.sort_unstable_by_key(|&(x, h)| (x, Reverse(h)));
    let mut stack = vec![];
    for (x, h) in b {
        match stack.last() {
            Some((_, ph)) if *ph < h => {
                stack.push((x, h));
            }
            None => {
                stack.push((x, h));
            }
            _ => {}
        }
    }

    for t in t {
        let pos = stack.partition_point(|&(x, h)| x + h < t);
        if pos == stack.len() {
            let (_, h) = stack[pos - 1];
            putln!(h);
        } else if pos == 0 {
            let (x, _) = stack[pos];
            putln!(t.saturating_sub(x));
        } else {
            let (_, h) = stack[pos - 1];
            let (x, _) = stack[pos];
            putln!(h.max(t.saturating_sub(x)));
        }
    }
}
