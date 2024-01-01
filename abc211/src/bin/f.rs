use itertools::Itertools;
use proconio::*;
use segtree::range_add_range_minimum_query;

fn main() {
    input! {n: usize}

    let mut st = range_add_range_minimum_query(100010);

    // (x, inout, low, high), in = 0, out = 1
    let mut edges = vec![];
    for _ in 0..n {
        input! {m: usize, e: [(usize, usize); m]}

        let mut ne = vec![];
        for v in e.windows(2) {
            let (lx, ly) = v[0];
            let (rx, ry) = v[1];
            if lx == rx {
                ne.push((lx, ly, ry));
            }
        }
        ne.sort();
        for (x, l, r) in ne {
            let (l, r) = (l.min(r), l.max(r));
            let min = st.prod(l, r);
            if min == 0 {
                edges.push((x, 0, l, r));
                st.apply_range(l, r, 1);
            } else {
                edges.push((x, 1, l, r));
                st.apply_range(l, r, -1);
            }
        }
    }

    edges.sort();
    edges.reverse();

    input! {q: usize, query: [(usize, usize); q]}
    let mut query = query
        .into_iter()
        .enumerate()
        .map(|(i, (x, y))| (x, y, i))
        .collect::<Vec<_>>();
    query.sort();
    let mut res = vec![0; q];
    for (x, y, i) in query {
        while let Some((px, inout, l, r)) = edges.pop() {
            if px > x {
                edges.push((px, inout, l, r));
                break;
            }

            if inout == 0 {
                st.apply_range(l, r, 1);
            } else {
                st.apply_range(l, r, -1);
            }
        }

        let min = st.prod(y, y + 1);
        res[i] = min;
    }

    println!("{}", res.into_iter().join("\n"))
}
