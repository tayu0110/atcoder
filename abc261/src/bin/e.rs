#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, mut c: i64, p: [(usize, i64); n]};

    let mut res = vec![0; n];
    for i in 0..30 {
        let mut xor_sum = -1;
        let mut now = (c >> i) & 1;
        let mut base = -1;
        for (j, &(t, a)) in p.iter().enumerate() {
            let q = (a >> i) & 1;
            if t == 1 {
                if q == 0 {
                    now = 0;
                    xor_sum = -1;
                    base = 0;
                } else if xor_sum >= 0 {
                    if base >= 0 {
                        now = base;
                    }
                    now ^= xor_sum;
                }
            } else if t == 2 {
                if q == 1 {
                    now = 1;
                    xor_sum = -1;
                    base = 1;
                } else if xor_sum >= 0 {
                    if base >= 0 {
                        now = base;
                    }
                    now ^= xor_sum;
                }
            } else {
                if xor_sum < 0 {
                    xor_sum = q;
                } else {
                    xor_sum ^= q;
                }
                if base >= 0 {
                    now = base;
                }
                now ^= xor_sum;
            }
            res[j] |= now << i;
            // eprintln!("i: {}, j: {}, res[j]: {}, now: {}", i, j, res[j], now);
        }
    }

    for v in res {
        println!("{}", v);
    }
}
