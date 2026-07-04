use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, usize); n]}

    let mut event = vec![vec![]; m + 1];
    let mut color = vec![0; n + 1];
    for (i, &(a, d, b)) in e.iter().enumerate() {
        color[a] += 1;
        event[d].push((b, i));
    }

    let mut kind = color.iter().filter(|c| **c > 0).count();
    for i in 1..=m {
        while let Some((b, i)) = event[i].pop() {
            let a = e[i].0;
            color[a] -= 1;
            if color[a] == 0 {
                kind -= 1;
            }
            color[b] += 1;
            if color[b] == 1 {
                kind += 1;
            }
        }

        println!("{kind}")
    }
}
