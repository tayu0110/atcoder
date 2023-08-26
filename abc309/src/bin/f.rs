use proconio::*;
use segtree::SegmentTree;
use std::collections::BTreeMap;

fn main() {
    input! {n: usize, mut p: [[usize; 3]; n]}
    let (cnt, map) = {
        let mut map = BTreeMap::new();
        for v in p.iter().flatten() {
            map.insert(*v, 0);
        }
        let mut cnt = 1;
        for (_, v) in map.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
        (cnt, map)
    };

    p.iter_mut().for_each(|v| {
        v.iter_mut().for_each(|v| {
            let k = *v;
            *v = *map.get(&k).unwrap()
        });
        v.sort()
    });
    p.sort();

    let mut t = vec![vec![]; cnt];
    for p in p {
        t[p[0]].push((p[1], p[2]));
    }

    let mut st = SegmentTree::new(cnt + 1, 0, |&l, &r| l.max(r));
    while let Some(t) = t.pop() {
        if t.is_empty() {
            continue;
        }

        for &(y, z) in &t {
            let nz = st.foldl(y + 1, cnt + 1);
            if nz > z {
                println!("Yes");
                return;
            }
        }

        for (y, z) in t {
            st.update_by(y, z, |&l, &r| l.max(r));
        }
    }

    println!("No")
}
