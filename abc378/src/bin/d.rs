use proconio::*;

fn solve(
    h: usize,
    w: usize,
    r: usize,
    c: usize,
    get_index: &impl Fn(usize, usize) -> usize,
    rem: usize,
    map: u128,
) -> usize {
    if rem == 0 {
        return 1;
    }

    let mut res = 0;
    for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
        let nr = r.wrapping_add(dr);
        let nc = c.wrapping_add(dc);
        if nr < h && nc < w {
            let index = get_index(nr, nc);
            if map & (1u128 << index) == 0 {
                res += solve(h, w, nr, nc, get_index, rem - 1, map | (1u128 << index));
            }
        }
    }
    res
}

fn main() {
    input! {h: usize, w: usize, k: usize, s: [marker::Bytes; h]}

    let get_index = |i: usize, j: usize| -> usize { i * w + j };
    let mut map = 0u128;
    for i in 0..h {
        for j in 0..w {
            let index = get_index(i, j);
            map |= ((s[i][j] == b'#') as u128) << index;
        }
    }

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            let index = get_index(i, j);
            if map & (1u128 << index) == 0 {
                res += solve(h, w, i, j, &get_index, k, map | (1u128 << index));
            }
        }
    }

    println!("{res}")
}
