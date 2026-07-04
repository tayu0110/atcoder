use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut prev = vec![usize::MAX; n];
    let mut pos = vec![usize::MAX; n + 1];
    for (i, &a) in a.iter().enumerate() {
        if pos[a - 1] != usize::MAX {
            prev[i] = pos[a - 1];
        }
        if pos[a] != usize::MAX {
            if prev[i] < usize::MAX {
                prev[i] = prev[i].max(pos[a]);
            } else {
                prev[i] = pos[a];
            }
        }
        pos[a] = i;
    }

    let mut pos = vec![usize::MAX; n + 1];
    let mut next = vec![usize::MAX; n];
    for (i, &a) in a.iter().enumerate().rev() {
        if pos[a - 1] != usize::MAX {
            next[i] = pos[a - 1];
        }
        pos[a] = i;
    }

    let mut res = 0usize;
    for i in 0..n {
        if prev[i] != usize::MAX && next[i] != usize::MAX {
            let prev = prev[i];
            let next = next[i];
            res += (next - i) * (i - prev);
        } else if prev[i] != usize::MAX {
            let prev = prev[i];
            res += (i - prev) * (n - i);
        } else if next[i] != usize::MAX {
            let next = next[i];
            res += (next - i) * (i + 1);
        } else {
            res += (i + 1) * (n - i);
        }
    }
    println!("{res}")
}
