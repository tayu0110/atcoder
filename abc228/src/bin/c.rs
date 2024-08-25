#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize, usize); n]}

    let p = p.iter().map(|(a, b, c)| a+b+c).collect_vec();
    let mut np = p.iter().enumerate().map(|(i, a)| (*a, i)).collect_vec();
    np.sort_by_key(|v| std::cmp::Reverse(*v));

    let mut now = vec![0; n];
    for (rank, &(_, i)) in np.iter().enumerate() {
        now[i] = rank+1;
    }

    for (i, v) in p.into_iter().enumerate() {
        if now[i] <= k {
            println!("Yes");
            continue;
        }

        let nv = v + 300;
        let (mut l, mut r) = (-1, n as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if np[m as usize].0 > nv {
                l = m;
            } else {
                r = m;
            }
        }

        if r < k as i32 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
