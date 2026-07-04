use ds::{EulerTourTree, MapMonoid};
use itertools::Itertools;
use proconio::*;

struct T;
impl MapMonoid for T {
    type M = usize;
    type Act = usize;

    fn e() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
    fn id() -> Self::Act {
        0
    }
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        l + r
    }
    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        m + act
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut ett = EulerTourTree::<T>::new(n);
    for i in 0..n {
        let mut size = vec![0; t[i].len()];
        for (j, &to) in t[i].iter().enumerate() {
            if to < i {
                size[j] = ett.tree_size(to);
            }
        }

        eprintln!("i: {i}, size: {size:?}");
        let sum = size.iter().sum::<usize>();
        ett.set(i, sum);
        for (j, &to) in t[i].iter().enumerate() {
            if to < i {
                ett.apply(to, &(sum - size[j] + 1));
                ett.link(i, to).ok();
            }
        }
    }

    for i in 0..n {
        for &to in &t[i] {
            ett.cut(i, to);
        }
    }
    println!("{}", (0..n).map(|i| ett.fold(i)).join(" "));
}
