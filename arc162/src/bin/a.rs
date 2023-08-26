use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut res = vec![];

    for _ in 0..t {
        input! {n: usize, p: [usize; n]}

        let mut t = p
            .iter()
            .copied()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect::<Vec<_>>();
        t.sort();
        let mut r = 0;

        for &(v, i) in &t {
            let tm = i + 1000;
            let mut s = vec![std::usize::MAX; n];
            s[i] = tm;
            for (_, &(_, ni)) in t.iter().take(v).rev().enumerate() {
                s[ni] = tm - ni;
            }
            let &min = s.iter().min().unwrap();
            if s[i] == min && s.into_iter().filter(|&v| v == min).count() == 1 {
                r += 1;
            }
        }

        res.push(r);
    }

    println!("{}", res.iter().join("\n"))
}
