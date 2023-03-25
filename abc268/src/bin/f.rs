use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, s: [String; n]}

    let mut s = s.into_iter().map(|s| {
        let (mut sx, mut sn) = (0, 0);
        for sc in s.chars() {
            if sc == 'X' {
                sx += 1;
            } else {
                sn += (sc as u8 - b'0') as usize;
            }
        }
        (sx, sn, s)
    }).collect_vec();

    s.sort_by(|(sx, sn, _), (tx, tn, _)| {
        (sx * tn).cmp(&(tx * sn))
    });

    let mut res = 0;
    let mut sum = 0;
    for (_, _, ns) in s {
        for c in ns.chars().rev() {
            if c == 'X' {
                res += sum;
            } else {
                sum += (c as u8 - b'0') as usize;
            }
        }
    }

    println!("{}", res);
}
