use matrix::Matrix;
use proconio::*;
use static_modint::{Mod998244353, StaticModint};
use std::collections::HashMap;

type Modint = StaticModint<Mod998244353>;

fn naive(n: usize, s: Vec<Vec<char>>) -> usize {
    let s = s
        .iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for mut i in 0..1 << n {
        let mut t = String::new();
        for _ in 0..n {
            t.push(if i & 1 != 0 { 'a' } else { 'b' });
            i >>= 1;
        }

        if s.iter().any(|s| t.contains(s)) {
            continue;
        }

        res += 1;
    }
    res
}

fn main() {
    input! {n: usize, m: usize, s: [marker::Chars; m]}
    const MAX_LEN: usize = 6;

    if n <= 20 {
        println!("{}", naive(n, s));
        return;
    }

    let s = s
        .iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<_>>();
    let mut node = HashMap::new();
    let mut cnt = 0;
    for mut i in 0..1 << MAX_LEN {
        let mut t = String::new();
        for _ in 0..MAX_LEN {
            t.push(if i & 1 != 0 { 'a' } else { 'b' });
            i >>= 1;
        }

        if s.iter().any(|s| t.contains(s)) {
            continue;
        }

        node.insert(t, cnt);
        cnt += 1;
    }

    if cnt == 0 {
        println!("0");
        return;
    }

    let mut map = HashMap::new();
    for (t, &id) in &node {
        let suf = t[1..].to_string();
        for c in vec!["a", "b"] {
            let nt = [suf.clone(), c.to_string()].concat();
            if let Some(nid) = node.get(&nt) {
                map.entry(id).or_insert(vec![]).push(nid);
            }
        }
    }

    let mut mat = Matrix::new(cnt, cnt);
    for (k, v) in map {
        for v in v {
            mat.set(k, *v, Modint::one());
        }
    }

    let p = mat.pow(n - 6);
    let mut res = Modint::zero();
    for i in 0..cnt {
        for j in 0..cnt {
            res += p.get(i, j);
        }
    }

    println!("{}", res);
}
