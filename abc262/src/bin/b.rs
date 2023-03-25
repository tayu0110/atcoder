#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]};

    let mut t = vec![vec![]; n];
    for (l, r) in p {
        t[l-1].push(r-1);
        t[r-1].push(l-1);
    }

    for v in t.iter_mut() {
        if v.is_empty() {
            continue;
        }
        v.sort();
    }

    let mut res = 0;
    for i in 0..n {
        for j in &t[i] {
            if *j <= i {
                continue;
            }
            for k in &t[*j] {
                if *k <= *j {
                    continue;
                }
                if !t[*k].is_empty() {
                    if let Ok(_) = t[*k].binary_search(&i) {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);
}
