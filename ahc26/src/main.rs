#![allow(dead_code)]
use fenwick_tree::FenwickTree;
use proconio::*;
use rand::{thread_rng, Rng};
use std::time::SystemTime;

const N: usize = 200;
const M: usize = 10;
const W: usize = N / M;
const Q: usize = 5000;

fn inversion_number(iter: impl IntoIterator<Item = usize>) -> usize {
    let mut ft = FenwickTree::new(201, 0);
    let mut res = 0;
    for i in iter {
        res += ft.get_sum(i, 201);
        ft.add(i, 1);
    }
    res
}

// (cost, query)
fn naive(rem_query: usize, mut b: Vec<Vec<usize>>) -> (usize, Vec<(usize, usize)>) {
    let mut query = vec![];
    let mut cost = 0;

    for _ in 0..rem_query {
        if b.iter().all(|v| v.is_empty()) {
            break;
        }

        let &min = b.iter().flatten().min().unwrap();
        let pos = b.iter().position(|v| v.contains(&min)).unwrap();

        let mut buf = vec![];
        while b[pos].last().unwrap() != &min {
            buf.push(b[pos].pop().unwrap());
        }

        if buf.is_empty() {
            b[pos].pop();
            query.push((min, 0));
        } else {
            let &v = buf.last().unwrap();
            let min = (0..M)
                .filter_map(|i| (i != pos).then_some(b[i].len()))
                .min()
                .unwrap();
            let mut topos = 0;
            for i in 0..M {
                if i != pos && b[i].len() == min {
                    topos = i;
                }
            }

            cost += buf.len() + 1;
            b[topos].extend(buf.into_iter().rev());
            query.push((v, topos + 1));
        }
    }

    (cost, query)
}

// (cost, query)
fn sorting(mut b: Vec<Vec<usize>>, costs: &mut [usize]) -> Option<(usize, Vec<(usize, usize)>)> {
    let mut query = vec![];
    let mut cost = 0;
    let mut rem = Q;
    let mut rng = thread_rng();
    let mut max = 200;
    let mut base = [0; M];
    let mut is_sorted = false;

    'B: while rem > 0 {
        if b.iter().all(|v| v.is_empty()) {
            break;
        }

        rem -= 1;

        let &min = b.iter().flatten().min().unwrap();
        let pos = b.iter().position(|v| v.contains(&min)).unwrap();
        if b[pos].last().unwrap() == &min {
            costs[min] = cost;
            query.push((b[pos].pop().unwrap(), 0));
            continue;
        }

        if max > 0 && !is_sorted {
            if let Some(topos) = b.iter().enumerate().position(|(i, v)| v.len() == base[i]) {
                let mut mpos = 0;
                while max > 0 {
                    if let Some(pos) = b.iter().position(|v| v.contains(&max)) {
                        mpos = pos;
                        let p = b[mpos].iter().position(|v| *v == max).unwrap();
                        if b[mpos][p..].windows(2).all(|v| v[0] < v[1]) {
                            max -= 1;
                        } else {
                            break;
                        }
                    } else {
                        is_sorted = true;
                        rem += 1;
                        continue 'B;
                    }
                }

                let mut buf = vec![];
                while b[mpos].last().unwrap() != &max {
                    buf.push(b[mpos].pop().unwrap());
                }
                buf.push(b[mpos].pop().unwrap());
                cost += buf.len() + 1;
                b[topos].extend(buf.into_iter().rev());
                base[topos] += 1;
                query.push((max, topos + 1));
                max -= 1;
                continue;
            }
        }

        let p = b[pos].iter().position(|v| *v == min).unwrap();
        let len = b[pos].len();
        let thresh = rng.gen_range(1..len - p);
        let buf = b[pos][len - thresh..].to_vec();
        b[pos].truncate(len - thresh);

        cost += buf.len() + 1;

        let tar = if let Some(pos) = b.iter().position(|v| v.is_empty()) {
            pos
        } else {
            let mut tar = 0;
            let mut score = f64::MAX;
            for i in (0..M).filter(|i| i != &pos) {
                let inv = inversion_number(buf.iter().rev().chain(b[i].iter().rev()).cloned());
                let height = buf.len() + b[i].len();
                let c = (inv as f64).log10() * (height as f64).log10();
                if c < score {
                    score = c;
                    tar = i;
                }
            }
            tar
        };

        query.push((*buf.last().unwrap(), tar + 1));
        b[tar].extend(buf.into_iter().rev());
    }

    b.iter().all(|v| v.is_empty()).then_some((cost, query))
}

fn main() {
    let now = SystemTime::now();
    input! {_: usize, _: usize, mut b: [[usize; W]; M]}
    let mut rng = thread_rng();

    let (mut cost, mut query) = naive(Q, b.clone());
    let mut loop_count = 0;
    let mut costs = vec![0; 201];

    while now.elapsed().unwrap().as_millis() < 1950 {
        let mut new = vec![0; 201];
        if let Some((nc, nq)) = sorting(b.clone(), &mut new) {
            if nq.len() < Q && nc < cost {
                cost = nc;
                query = nq;
                costs = new;
            } else {
                let c = rng.gen_range(0..201);
                if new[c] < costs[c] {
                    let mut b = b.clone();
                    let mut keep = vec![];
                    for (v, i) in nq {
                        if v == c {
                            break;
                        }
                        if i == 0 {
                            for i in 0..M {
                                if !b[i].is_empty() && b[i].last().unwrap() == &v {
                                    b[i].pop();
                                    break;
                                }
                            }
                        } else {
                            for j in 0..M {
                                if b[j].contains(&v) {
                                    let pos = b[j].iter().position(|w| *w == v).unwrap();
                                    let t = b[j][pos..].to_vec();
                                    b[i - 1].extend(t);
                                    b[j].truncate(pos);
                                    break;
                                }
                            }
                        }
                        keep.push((v, i));
                    }

                    if let Some((nc, nq)) = sorting(b, &mut new) {
                        keep.extend(nq);
                        if keep.len() < Q && nc < cost {
                            cost = nc;
                            query = keep;
                            costs = new;
                        }
                    }
                }
            }
        }
        loop_count += 1;
    }

    for (v, i) in query {
        println!("{v} {i}");
    }

    eprintln!("loop_count: {loop_count}");
}
