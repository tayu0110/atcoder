#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize); m]}

    let mut map = std::collections::BTreeMap::new();
    for (a, _) in &p {
        map.insert(*a, 0);
    }
    let mut cnt = 0;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    for (a, _) in p.iter_mut() {
        let na = *a;
        *a = *map.get(&na).unwrap();
    }

    let mut t = vec![vec![]; cnt];
    for (a, b) in p {
        t[a].push(b);
    }

    let mut set = std::collections::HashSet::new();
    set.insert(n);

    for v in t {
        if v.is_empty() {
            continue;
        }

        // eprintln!("v: {:?}", v);

        let mut buf = vec![];
        for w in &v {
            if *w > 0 && set.contains(&(*w-1)) {
                buf.push(*w);
            }
            if *w < 2*n && set.contains(&(*w+1)) {
                buf.push(*w);
            }
        }

        // eprintln!("buf: {:?}", buf);
        for w in v {
            set.remove(&w);
        }

        for w in buf {
            set.insert(w);
        }

        // eprintln!("set: {:?}", set);
    }

    println!("{}", set.len());
}
