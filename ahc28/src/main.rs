use std::time::SystemTime;

use proconio::*;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use rustc_hash::FxHashSet;
use suffix_array::SuffixArray;

const N: usize = 15;
const M: usize = 200;

type HashSet<V> = FxHashSet<V>;

fn compress_sa(t: &[String], rng: &mut ThreadRng) -> Vec<String> {
    let s = t.concat();
    let sa_ = SuffixArray::new(&s);
    let sa = sa_.iter().map(|&sa| sa as usize).collect::<Vec<_>>();
    let mut perm = (0..t.len()).collect::<Vec<_>>();
    perm.shuffle(rng);
    let mut si = vec![usize::MAX; t.len()];
    for (i, &sa) in sa.iter().enumerate() {
        if sa % 5 == 0 {
            si[sa / 5] = i;
        }
    }
    let lcpa = sa_.lcp_array();
    let mut next = vec![usize::MAX; t.len()];
    let mut buf = t.to_vec();

    for p in perm {
        let i = si[p];
        let (mut pl, mut pr) = (t[p].len(), t[p].len());
        for w in 1.. {
            if i + w < sa.len() && i >= w {
                pl = lcpa[i - w].min(pl);
                pr = lcpa[i + w - 1].min(pr);
                if sa[i - w] / 5 != p && pl > pr && next[sa[i - w] / 5] == usize::MAX {
                    next[sa[i - w] / 5] = p;
                    buf[p] = buf[p][pl..].to_owned();
                    break;
                } else if sa[i + w] / 5 != p && next[sa[i + w] / 5] == usize::MAX {
                    next[sa[i + w] / 5] = p;
                    buf[p] = buf[p][pr..].to_owned();
                    break;
                }
            } else if i + w < sa.len() {
                pr = lcpa[i + w - 1].min(pr);
                if sa[i + w] / 5 != p {
                    next[sa[i + w] / 5] = p;
                    buf[p] = buf[p][pr..].to_owned();
                    break;
                }
            } else {
                pl = lcpa[i - w].min(pl);
                if sa[i - w] / 5 != p {
                    next[sa[i - w] / 5] = p;
                    buf[p] = buf[p][pl..].to_owned();
                    break;
                }
            }
        }
    }

    let mut used = vec![false; t.len()];
    let mut res = vec![];
    for i in 0..t.len() {
        if used[i] {
            continue;
        }

        let mut now = i;
        while now < usize::MAX && !used[now] {
            used[now] = true;
            res.push(buf[now].clone());
            now = next[now];
        }
    }

    res
}

#[allow(unused)]
fn compress(t: &[String]) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut hashes: Vec<HashSet<String>> = vec![];
    'b: for t in t {
        if res.iter().any(|s| s.contains(t)) {
            continue;
        }
        for tail in (1..=t.len()).rev() {
            let ns = t[..tail].to_owned();
            for i in 0..res.len() {
                if hashes[i].contains(&ns) {
                    for c in t[tail..].chars() {
                        res[i].push(c);
                    }
                    hashes[i].clear();
                    for j in 0..res[i].len() {
                        hashes[i].insert(res[i][j..].to_owned());
                    }
                    continue 'b;
                }
            }
        }

        res.push(t.clone());
        let mut h = HashSet::default();
        for i in 0..t.len() {
            h.insert(t[i..].to_owned());
        }
        hashes.push(h);
    }

    res
}

fn main() {
    let mut tm = SystemTime::now();
    input! {_: usize, _: usize, si: usize, sj: usize, a: [marker::Bytes; N], mut t: [String; M]}
    if cfg!(debug_assertions) {
        tm = SystemTime::now();
    }

    {
        let mut cnt = 0;
        'b: while cnt < t.len() {
            for j in 0..t.len() {
                if cnt == j {
                    continue;
                }
                if t[j] == t[cnt] {
                    t.remove(cnt);
                    continue 'b;
                }
            }

            cnt += 1;
        }
    }

    let mut rng = thread_rng();

    let mut place = vec![vec![]; 26];
    for i in 0..N {
        for j in 0..N {
            let index = (a[i][j] - b'A') as usize;
            place[index].push((i, j));
        }
    }

    let mut score = usize::MAX;
    let mut res = vec![];
    while tm.elapsed().unwrap().as_millis() < 50 {
        let (mut r, mut c) = (si, sj);
        let mut tmp = vec![];
        let mut tmp_score = 0;
        let ct = compress(&t);
        for t in ct {
            for t in t.bytes() {
                let mut min = usize::MAX;
                let index = (t - b'A') as usize;
                let mut next = (r, c);
                for &(nr, nc) in &place[index] {
                    if r.abs_diff(nr) + c.abs_diff(nc) + 1 < min {
                        next = (nr, nc);
                        min = r.abs_diff(nr) + c.abs_diff(nc) + 1;
                    }
                }

                tmp.push(next);
                tmp_score += r.abs_diff(next.0) + c.abs_diff(next.1) + 1;
                (r, c) = next;
            }
        }

        if tmp.len() <= 5000 && tmp_score < score {
            score = tmp_score;
            res = tmp;
        }

        t.shuffle(&mut rng);
    }

    while tm.elapsed().unwrap().as_millis() < 1950 {
        let (mut r, mut c) = (si, sj);
        let mut tmp = vec![];
        let mut tmp_score = 0;
        // let ct = compress(&t);
        let ct = compress_sa(&t, &mut rng);
        for t in ct {
            for t in t.bytes() {
                let mut min = usize::MAX;
                let index = (t - b'A') as usize;
                let mut next = (r, c);
                for &(nr, nc) in &place[index] {
                    if r.abs_diff(nr) + c.abs_diff(nc) + 1 < min {
                        next = (nr, nc);
                        min = r.abs_diff(nr) + c.abs_diff(nc) + 1;
                    }
                }

                tmp.push(next);
                tmp_score += r.abs_diff(next.0) + c.abs_diff(next.1) + 1;
                (r, c) = next;
            }
        }

        if tmp.len() <= 5000 && tmp_score < score {
            score = tmp_score;
            res = tmp;
        }

        t.shuffle(&mut rng);
    }

    assert!(res.len() <= 5000);

    for (r, c) in res {
        println!("{r} {c}");
    }
}
