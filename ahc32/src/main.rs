use std::{cmp::Reverse, ptr::copy_nonoverlapping};

use itertools::Itertools;
use proconio::*;
use rand::{seq::SliceRandom, thread_rng};
use static_modint::{Mod998244353, StaticModint};

const N: usize = 9;
const M: usize = 20;
const K: usize = 81;
const MOD: u64 = 998244353;

type Modint = StaticModint<Mod998244353>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct State {
    index: usize,
    prev: usize,
    score: u64,
    instruction: (u8, u8, u8),
    a: [[Modint; N]; N],
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve(a: [[Modint; N]; N], s: &[[Modint; 9]]) -> Vec<(u8, u8, u8)> {
    let tm = std::time::SystemTime::now();
    let mut rng = thread_rng();
    const MAX_STATES_NUM_FIXED: usize = 50;
    const MAX_STATES_NUM_RANDOM: usize = 10;
    // const MAX_CHOOSE_COORDINATE: usize = 70;

    let mut history = vec![State {
        index: 0,
        prev: usize::MAX,
        score: a.iter().flatten().map(|a| a.val() as u64).sum(),
        instruction: (0, 0, 0),
        a,
    }];
    let mut states = vec![0];

    let mut res = history[0].clone();

    for _ in 0..K {
        let mut new = vec![];
        for now in states {
            let State { score, a, .. } = history[now].clone();
            for (k, s) in s.iter().enumerate() {
                for i in 0..N - 2 {
                    for j in 0..N - 2 {
                        let mut score = score;
                        let mut a = a.clone();
                        for di in 0..3 {
                            for dj in 0..3 {
                                a[i + di][j + dj] += s[di * 3 + dj];
                                if a[i + di][j + dj].val() < s[di * 3 + dj].val() {
                                    score -= MOD - s[di * 3 + dj].val() as u64;
                                } else {
                                    score += s[di * 3 + dj].val() as u64;
                                }
                            }
                        }

                        let id = history.len();
                        history.push(State {
                            index: id,
                            prev: now,
                            score,
                            instruction: (k as u8, i as u8, j as u8),
                            a,
                        });
                        new.push(id);
                    }
                }
            }
        }

        new.sort_unstable_by_key(|i| Reverse(history[*i].score));
        res = res.min(history[new[0]].clone());
        if new.len() > MAX_STATES_NUM_FIXED {
            let (s, t) = new.split_at(MAX_STATES_NUM_FIXED);
            states = s.to_vec();
            states.extend(t.choose_multiple(&mut rng, MAX_STATES_NUM_RANDOM).cloned());
        } else {
            states = new;
        }
    }

    eprintln!("tm: {} millis", tm.elapsed().unwrap().as_millis());
    let mut now = res.index;
    let mut result = vec![];
    while history[now].prev != usize::MAX {
        result.push(history[now].instruction);
        now = history[now].prev;
    }
    result
}

fn main() {
    input! {_: usize, _: usize, _: usize, mut a: [u32; N * N], s: [u32; M * 9]}

    let mut na = [[Modint::zero(); N]; N];
    let mut ns = [[Modint::zero(); 9]; M];
    unsafe {
        copy_nonoverlapping(a.as_ptr(), na.as_mut_ptr() as _, N * N);
        copy_nonoverlapping(s.as_ptr(), ns.as_mut_ptr() as _, M * 9);
    }
    let result = solve(na, &ns);

    println!("{}", result.len());
    println!(
        "{}",
        result
            .into_iter()
            .map(|(m, p, q)| format!("{m} {p} {q}"))
            .join("\n")
    )
}
