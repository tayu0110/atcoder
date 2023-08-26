use itertools::Itertools;
use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize}

        if n == 1 {
            println!("Yes");
            println!("1");
            continue;
        }

        if n == 2 {
            println!("No");
            continue;
        }

        let mut res = vec![2, 3, 6].into_iter().collect::<BTreeSet<_>>();
        while res.len() < n {
            let mut r = 0;
            for &now in &res {
                let (s, t) = (now + 1, now * (now + 1));
                if !res.contains(&s) && !res.contains(&t) {
                    r = now;
                    break;
                }
            }

            res.remove(&r);
            res.insert(r + 1);
            res.insert(r * (r + 1));
        }

        println!("Yes");
        println!("{}", res.iter().join(" "))
    }
}
