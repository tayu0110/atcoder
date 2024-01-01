use proconio::*;
use rustc_hash::FxHashSet;

type HashSet<V> = FxHashSet<V>;

fn main() {
    input! {n: usize, q: usize, c: [usize; n]}

    let mut c = c
        .into_iter()
        .map(|c| {
            let mut set = HashSet::default();
            set.insert(c);
            set
        })
        .collect::<Vec<_>>();

    for _ in 0..q {
        input! {a: usize, b: usize}

        let (a, b) = (a - 1, b - 1);

        if c[a].is_empty() {
            println!("{}", c[b].len());
        } else if c[b].is_empty() {
            println!("{}", c[a].len());
            c.swap(a, b);
        } else if c[a].len() < c[b].len() {
            let mut v = HashSet::default();
            std::mem::swap(&mut v, &mut c[a]);
            c[b].extend(v);
            println!("{}", c[b].len());
        } else {
            let mut v = HashSet::default();
            std::mem::swap(&mut v, &mut c[b]);
            c[a].extend(v);
            c.swap(a, b);
            println!("{}", c[b].len())
        }
    }
}
