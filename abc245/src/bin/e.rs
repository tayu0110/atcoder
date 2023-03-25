#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; n], c: [usize; m], d: [usize; m]}

    let mut p = a.into_iter().zip(b.into_iter()).collect::<Vec<(usize, usize)>>();
    let mut q = c.into_iter().zip(d.into_iter()).collect::<Vec<(usize, usize)>>();
    p.sort();
    q.sort();

    let mut nt = std::collections::BTreeMap::new();
    while let Some((a, b)) = p.pop() {
        while let Some((c, d)) = q.pop() {
            if c < a {
                q.push((c, d));
                break;
            }

            *nt.entry(d).or_insert(0) += 1;
        }

        if let Some((k, _)) = nt.range(b..).next() {
            let k = *k;
            *nt.entry(k).or_insert(0) -= 1;

            if nt.get(&k).unwrap() == &0 {
                nt.remove(&k);
            }
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
