use std::io::stdin;

use proconio::*;

fn main() {
    input! {_: usize, w: usize}

    let s = stdin().lines().collect::<Result<Vec<_>, _>>().unwrap();

    let mut res = 0usize;
    let mut ingot = vec![0u16; w];
    for s in s.into_iter().rev() {
        let mut orb = 0;
        for (j, s) in s.bytes().enumerate().rev() {
            if s == b'J' {
                res += orb * ingot[j] as usize;
            } else if s == b'O' {
                orb += 1;
            } else {
                ingot[j] += 1;
            }
        }
    }

    println!("{res}")
}
