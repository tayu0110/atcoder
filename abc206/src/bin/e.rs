#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {l: usize, r: usize}

    let mut d = vec![std::usize::MAX; r+1];
    for i in 2..=r {
        if d[i] == std::usize::MAX {
            for j in (2..=r).take_while(|j| i**j <= r) {
                d[i*j] = i;
            }
        }
    }

    let mut res = 0;
    for now in l..=r {
        let mut buf = vec![];
        let mut k = now;
        while d[k] != std::usize::MAX {
            if !buf.iter().any(|v| *v == d[k]) {
                buf.push(d[k]);
            }
            k /= d[k];
        }
        if !buf.iter().any(|v| *v == k) {
            buf.push(k);
        }

        let m = buf.len();
        let mut coprime = (r - now) as i64;
        for i in 1..(1 << m) {
            let mut t = 1;
            for j in 0..m {
                if i & (1 << j) != 0 {
                    t *= buf[j];
                }
            }

            let k = (r / t - now / t) as i64;
            if (i as i32).count_ones() % 2 == 0 {
                coprime += k;
            } else {
                coprime -= k;
            }
        }

        res += coprime as usize + r / now - 1;
    }

    println!("{}", (r-l+1) * (r-l+1) - res*2 - (r-l+1));
}
