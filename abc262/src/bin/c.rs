#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]};
    let a = a.into_iter().map(|v| v-1).collect::<Vec<_>>();

    let mut res = 0usize;
    let mut t = vec![std::collections::HashSet::new(); n];
    let mut k = 0usize;
    for (i, v) in a.into_iter().enumerate() {
        if i == v {
            res += k;
            k += 1;
        } else {
            t[v].insert(i);
            if t[i].contains(&(v)) {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
