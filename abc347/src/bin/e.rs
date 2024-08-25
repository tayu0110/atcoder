use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, x: [usize; q]}

    let mut set = vec![0u8; n + 1];
    let mut len = 0;
    let mut history = Vec::with_capacity(q + 1);
    history.push(0);
    let mut pos = vec![vec![]; n + 1];
    for (i, x) in x.into_iter().enumerate() {
        len -= set[x] as usize;
        set[x] ^= 1;
        len += set[x] as usize;
        history.push(len + history.last().unwrap());
        pos[x].push(i);
    }

    println!(
        "{}",
        pos.into_iter()
            .skip(1)
            .map(|mut pos| {
                if pos.is_empty() {
                    0
                } else {
                    pos.push(q);
                    pos.chunks_exact(2)
                        .map(|v| history[v[1]] - history[v[0]])
                        .sum::<usize>()
                }
            })
            .join(" ")
    )
}
