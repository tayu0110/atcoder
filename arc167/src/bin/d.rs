use std::collections::HashMap;

use itertools::Itertools;
use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut p: [usize; n]}
        p.iter_mut().for_each(|v| *v -= 1);

        let mut uf = UnionFind::new(n);
        for i in 0..n {
            uf.merge(i, p[i]);
        }

        {
            let r = uf.root(0);
            if (0..n).map(|i| uf.root(i)).all(|root| root == r) {
                println!("{}", p.iter().map(|v| v + 1).join(" "));
                continue;
            }
        }

        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut rev = vec![0; n];
        let mut group: Vec<Vec<usize>> = vec![];
        for i in 0..n {
            let r = uf.root(p[i]);
            if let Some(&index) = map.get(&r) {
                group[index].push(p[i]);
                rev[p[i]] = index;
            } else {
                map.insert(r, group.len());
                rev[p[i]] = group.len();
                group.push(vec![p[i]]);
            }
        }

        let mut min = usize::MAX;
        for i in 0..group[0].len() {
            min = min.min(group[0][i]);
        }
    }
}
