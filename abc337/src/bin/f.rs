use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, mut c: [usize; n]}
    c.iter_mut().for_each(|c| *c -= 1);

    let c = [c.clone(), c].concat();

    let mut used_ball = vec![VecDeque::new(); n];
    let mut used_box = 0;
    let mut stay = BinaryHeap::new();
    let mut stay_ball = vec![VecDeque::new(); n];
    let mut used_set = HashSet::new();
    for (i, &c) in c[..n].iter().enumerate() {
        if used_ball[c].len() % k != 0 {
            used_ball[c].push_back(i);
            used_set.insert(i);
        } else if used_box < m {
            used_box += 1;
            used_ball[c].push_back(i);
            used_set.insert(i);
        } else {
            stay.push(Reverse(i));
            stay_ball[c].push_back(i);
        }
    }

    if stay.is_empty() {
        println!("{}", (0..n).map(|_| n).join("\n"));
        return;
    }

    println!("{}", n - stay.len());
    let mut sl = stay.len();
    let (mut l, mut r) = (0, n);
    for _ in 1..n {
        if used_ball[c[l]].pop_front().is_some() {
            used_set.remove(&l);
            if used_ball[c[l]].is_empty() {
                while let Some(Reverse(p)) = stay.pop() {
                    if p < l {
                        continue;
                    }
                    if used_set.contains(&p) {
                        continue;
                    }

                    let nc = c[p];
                    for _ in 0..k {
                        if let Some(b) = stay_ball[nc].pop_front() {
                            used_ball[nc].push_back(b);
                            used_set.insert(b);
                            sl -= 1;
                        } else {
                            break;
                        }
                    }
                    break;
                }
            } else if let Some(&f) = used_ball[c[l]].front() {
                while let Some(&Reverse(s)) = stay.peek() {
                    if s < l {
                        stay.pop();
                        continue;
                    }
                    if used_set.contains(&s) {
                        stay.pop();
                        continue;
                    }

                    if s < f {
                        while let Some(f) = used_ball[c[l]].pop_back() {
                            stay.push(Reverse(f));
                            stay_ball[c[l]].push_front(f);
                            used_set.remove(&f);
                            sl += 1;
                        }
                        let Reverse(s) = stay.pop().unwrap();
                        let nc = c[s];
                        for _ in 0..k {
                            if let Some(b) = stay_ball[nc].pop_front() {
                                used_ball[nc].push_back(b);
                                used_set.insert(b);
                                sl -= 1;
                            } else {
                                break;
                            }
                        }
                    }
                    break;
                }
            }
        } else {
            sl -= 1;
        }

        if used_ball[c[r]].len() % k != 0 {
            used_ball[c[r]].push_back(r);
            used_set.insert(r);
        } else {
            stay.push(Reverse(r));
            stay_ball[c[r]].push_back(r);
            sl += 1;
        }

        l += 1;
        r += 1;

        println!("{}", n - sl);
    }
}
