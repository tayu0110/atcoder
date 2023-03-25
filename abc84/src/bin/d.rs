#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    const MAX: usize = 100010;
    let mut d = vec![std::usize::MAX; MAX];
    for now in 2..MAX {
        if d[now] == std::usize::MAX {
            for i in (2..MAX).take_while(|i| now**i < MAX) {
                d[now * i] = now;
            }
        }
    }

    let mut like = vec![];
    for i in 3..MAX {
        if d[i] == std::usize::MAX && d[(i+1)/2] == std::usize::MAX {
            like.push(i);
        }
    }

    input! {q: usize, p: [(usize, usize); q]}

    let binary_search = |t: usize| {
        let (mut l, mut r) = (0, like.len());

        while r - l > 1 {
            let m = (r + l) / 2;
            if like[m] <= t {
                l = m;
            } else {
                r = m;
            }
        }

        if like[l] <= t {
            Ok(l)
        } else {
            Err(())
        }
    };

    for (l, r) in p {
        if let Ok(r) = binary_search(r) {
            let mut res = r + 1;
            if let Ok(l) = binary_search(l-1) {
                res -= l + 1;
            }

            println!("{}", res);
        } else {
            println!("0");
        }
    }
}
