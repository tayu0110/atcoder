#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, p: [[i64; 3]; n]}

    let mut res = 0;
    for i in 0..(1 << 3) {
        let mut buf = vec![];
        for v in &p {
            let mut sum = 0;
            for (j, v) in v.iter().enumerate().take(3) {
                if i & (1 << j) == 0 {
                    sum -= v;
                } else {
                    sum += v;
                }
            }

            buf.push(sum);
        }

        buf.sort_by_key(|v| std::cmp::Reverse(*v));
        let r = buf.iter().take(m).sum::<i64>();

        res = std::cmp::max(res, r);
    }

    println!("{}", res);
}
