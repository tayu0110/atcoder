use std::collections::BinaryHeap;

use proconio::*;

struct S {
    a: usize,
    shift: usize,
}

impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for S {}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.a as u128 * (1u128 << other.shift)).cmp(&(other.a as u128 * (1u128 << self.shift)))
    }
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, k: usize, x: usize, a: [usize; n]}

        let mut cnt = 0;
        let mut nt = a
            .into_iter()
            .map(|a| S { a, shift: 0 })
            .collect::<BinaryHeap<_>>();

        let mut ret = vec![];
        while let Some(s) = nt.pop() {
            if cnt + (1 << s.shift) >= k {
                ret.push((s.a, s.shift + 1, (k - cnt) * 2));
                ret.push((s.a, s.shift, (1 << s.shift) - (k - cnt)));
                while let Some(s) = nt.pop() {
                    ret.push((s.a, s.shift, 1 << s.shift));
                }
                break;
            }
            cnt += 1 << s.shift;
            nt.push(S {
                a: s.a,
                shift: s.shift + 1,
            })
        }

        ret.sort_unstable_by_key(|(a, shift, _)| S {
            a: *a,
            shift: *shift,
        });
        let mut cnt = 0;
        while let Some((a, shift, c)) = ret.pop() {
            cnt += c;
            if cnt >= x {
                println!("{}", a as f64 / (1 << shift) as f64);
                break;
            }
        }
    }
}
