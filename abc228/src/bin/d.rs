#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {q: usize, p: [(usize, usize); q]}
    const N: usize = 1 << 20;

    let mut a = vec![std::usize::MAX; N];
    let mut set = std::collections::BTreeSet::new();

    for i in 0..N {
        set.insert(i);
    }

    for (t, x) in p {
        let h = x % N;

        if t == 1 {
            if let Some(&i) = set.range(h..).next() {
                a[i] = x;
                set.remove(&i);
            } else if set.is_empty() {
                a[h] = x;
            } else {
                let i = *set.iter().next().unwrap();
                a[i] = x;
                set.remove(&i);
            }
        } else if a[x % N] == std::usize::MAX {
            println!("-1");
        } else {
            println!("{}", a[x % N]);
        }
    }
}
