#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn solve(k: usize, a: &Vec<usize>, b: &Vec<usize>) -> i32 {
    let m = b.len();

    let mut res = 0;
    let a = a.into_iter().scan(0, |s, v| { *s += v; Some(*s) }).collect_vec();
    let b = b.into_iter().scan(0, |s, v| { *s += v; Some(*s) }).collect_vec();

    for (i, t) in a.into_iter().enumerate() {
        if t > k {
            break;
        }

        res = std::cmp::max(res, i as i32+1);

        let k = k - t;
        let (mut l, mut r) = (-1, m as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if b[m as usize] <= k {
                l = m;
            } else {
                r = m;
            }
        }

        if l < 0 {
            continue;
        }

        res = std::cmp::max(res, i as i32 +l+2);
    }

    res
}

#[fastout]
fn main() {
    input! {n: usize, m: usize, k: usize, a: [usize; n], b: [usize; m]}
    
    println!("{}", std::cmp::max(0, std::cmp::max(solve(k, &a, &b), solve(k, &b, &a))));
}
