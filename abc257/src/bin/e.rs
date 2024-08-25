#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, c: [usize; 9]};

    let mut dp = vec![(-1, 0); n+1];
    dp[n] = (0, 0);
    let mut max = (0, 0, 0);
    for now in (0..n+1).rev() {
        let (nn, _) = dp[now];
        if nn < 0 {
            continue;
        }
        for i in 0..9 {
            if c[i] > now {
                continue;
            }
            let to = now - c[i];
            let (tn, prev) = dp[to];
            if tn > nn+1 {
                continue;
            } else if tn < nn+1 {
                dp[to] = (nn+1, i);
            } else if prev < i {
                dp[to] = (nn+1, i);
            }
        }

        let l = (dp[now].0, dp[now].1, now);
        max = std::cmp::max(max, l);
    }

    if max.0 == 0 {
        println!("0");
        std::process::exit(0);
    }

    // eprintln!("{:?}", max);
    let mut res = String::new();
    let (len, _, _) = max;
    for i in 0..n+1 {
        let (nlen, _) = dp[i];
        if nlen != len {
            continue;
        }
        let mut now = i;
        let mut buf = String::new();
        while now < n {
            let (_, i) = dp[now];
            buf.push_str((i+1).to_string().as_str());
            now += c[i];
        }
        res = std::cmp::max(res, buf);
    }

    println!("{}", res);
}
