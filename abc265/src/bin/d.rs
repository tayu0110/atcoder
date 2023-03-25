#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: usize, q: usize, rr: usize, a: [usize; n]};

    let mut ck = std::collections::HashSet::new();
    let a = a.into_iter().fold(vec![0], |mut v, c| {
        let nc = c + *v.last().unwrap();
        ck.insert(nc);
        v.push(nc);
        v
    });

    let (mut l, mut r) = (0, 0);
    while l < n {
        while r < n && a[r] - a[l] < p {
            r += 1;
        }

        if a[r] - a[l] == p {
            if ck.contains(&(q + a[r])) && ck.contains(&(q + rr + a[r])) {
                println!("Yes");
                std::process::exit(0);
            }
        }

        l += 1;
        if r < l {
            r = l;
        }
    }

    println!("No");
}
