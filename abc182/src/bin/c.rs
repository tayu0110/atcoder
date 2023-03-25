use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: Chars}
    let n = n.into_iter().map(|c| (c as u8 - b'0') as usize).collect_vec();

    let len = n.len();
    let mut res = len;
    for i in 1..(1 << len) {
        let mut sum = 0;
        for j in 0..len {
            if i & (1 << j) != 0 {
                sum += n[j];
            }
        }

        if sum % 3 == 0 {
            res = std::cmp::min(res, len - (i as i32).count_ones() as usize);
        }
    }

    if res == len {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
