use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [usize; n], query: [i64; q]}

    let mut ret = vec![(usize::MAX, usize::MAX); q];
    let mut query = query
        .into_iter()
        .enumerate()
        .map(|(i, q)| (q, i))
        .collect::<Vec<_>>();
    query.sort_unstable();

    let mut i = 0;
    let mut ft = FenwickTree::<Addition<i64>>::new(n + 1);
    for &a in &a {
        ft.add(a, 1);
    }

    let mut position = vec![vec![]; n + 1];
    for (i, &a) in a.iter().enumerate() {
        position[a].push(i);
    }

    let mut seen = 0;
    for (index, &a) in a.iter().enumerate() {
        ft.add(a, -1);
        let sum = ft.fold(..a);
        let mut left = 0;
        while i < n && query[i].0 < seen + sum {
            let (k, j) = query[i];
            let (mut l, mut r) = (0, a);
            while r - l > 1 {
                let m = (r + l) / 2;
                if k <= seen + ft.fold(left..m) {
                    r = m;
                } else {
                    l = m;
                }
            }

            seen += ft.fold(left..l);
            left = l;

            let pos = position[r].partition_point(|&p| p < index);
            ret[j] = (index + 1, position[r][(k - seen - sum) as usize + pos] + 1);

            i += 1;
        }
    }
}
