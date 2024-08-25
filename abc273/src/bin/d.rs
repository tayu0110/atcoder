#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {h: i64, w: i64, rs: i64, cs: i64, n: usize, wall: [(i64, i64); n], q: usize, p: [(char, i64); q]}

    let mut row = std::collections::HashMap::new();
    let mut col = std::collections::HashMap::new();

    for (r, c) in wall {
        row.entry(r).or_insert(vec![]).push(c);
        col.entry(c).or_insert(vec![]).push(r);
    }
    for (_, v) in row.iter_mut() {
        v.sort();
    }
    for (_, v) in col.iter_mut() {
        v.sort();
    }

    let (mut nr, mut nc) = (rs, cs);
    for (d, rep) in p {
        if d == 'U' || d == 'D' {
            if !col.contains_key(&nc) {
                if d == 'U' {
                    nr = std::cmp::max(1, nr - rep);
                } else {
                    nr = std::cmp::min(h, nr + rep);
                }
            } else {
                let v = col.get(&nc).unwrap();
                let (mut l, mut r) = (-1, v.len() as i64);
                while r - l > 1 {
                    let m = (r + l) / 2;
                    if v[m as usize] > nr {
                        r = m;
                    } else {
                        l = m;
                    }
                }

                if d == 'U' {
                    if l < 0 {
                        nr = std::cmp::max(nr - rep, 1);
                    } else {
                        nr = std::cmp::max(nr - rep, std::cmp::max(1, v[l as usize] + 1));
                    }
                } else {
                    nr = std::cmp::min(nr + rep, v.get(r as usize).unwrap_or(&(h+1)) - 1);
                }
            }
        } else if !row.contains_key(&nr) {
            if d == 'L' {
                nc = std::cmp::max(1, nc - rep);
            } else {
                nc = std::cmp::min(w, nc + rep);
            }
        } else {
            let v = row.get(&nr).unwrap();
            let (mut l, mut r) = (-1, v.len() as i64);
            while r - l > 1 {
                let m = (r + l) / 2;
                if v[m as usize] > nc {
                    r = m;
                } else {
                    l = m;
                }
            }

            if d == 'L' {
                if l < 0 {
                    nc = std::cmp::max(nc - rep, 1);
                } else {
                    nc = std::cmp::max(nc - rep, std::cmp::max(1, v[l as usize] + 1));
                }
            } else {
                nc = std::cmp::min(nc + rep, v.get(r as usize).unwrap_or(&(w+1)) - 1);
            }
        }

        println!("{} {}", nr, nc);
    }
}
