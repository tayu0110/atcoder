use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}

    let mut ret = vec![0u8; n + 1];
    for y in (1..).take_while(|&y| y * y < n) {
        for x in (1..y).take_while(|&x| x * x + y * y <= n) {
            let i = x * x + y * y;
            ret[i] = ret[i].saturating_add(1);
        }
    }

    let ret = ret
        .into_iter()
        .enumerate()
        .filter_map(|(i, r)| (r == 1).then_some(i))
        .collect::<Vec<_>>();
    println!("{}", ret.len());
    println!("{}", ret.iter().join(" "))
}
