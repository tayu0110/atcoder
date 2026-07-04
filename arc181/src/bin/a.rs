use cpio::*;

fn solve(a: &[u32]) -> u8 {
    let cnt = a
        .iter()
        .enumerate()
        .filter(|&(i, &a)| a == i as u32 + 1)
        .count();

    let n = a.len();
    if cnt == n {
        return 0;
    }

    let mut max = 0;
    for (i, &a) in a.iter().enumerate() {
        if a == i as u32 + 1 && max == i as u32 {
            return 1;
        }
        max = max.max(a);
    }

    2 + (a[0] == n as u32 && a[n - 1] == 1) as u8
}

fn main() {
    scan! {t: usize}

    for _ in 0..t {
        scan! {n: usize, a: [u32; n]}
        putln!(solve(&a));
    }
}
