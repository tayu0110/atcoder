use proconio::*;

fn rot(r: usize, c: usize) -> (usize, usize) {
    match (r, c) {
        (1, 0) => (0, !0),
        (0, 1) => (1, 0),
        (0, usize::MAX) => (!0, 0),
        (usize::MAX, 0) => (0, 1),
        _ => unreachable!(),
    }
}

fn unti_rot(r: usize, c: usize) -> (usize, usize) {
    match (r, c) {
        (0, usize::MAX) => (1, 0),
        (1, 0) => (0, 1),
        (usize::MAX, 0) => (0, usize::MAX),
        (0, 1) => (usize::MAX, 0),
        _ => unreachable!(),
    }
}

fn main() {
    input! {h: usize, w: usize, n: usize}

    let mut grid = vec![vec!['.'; w]; h];
    let (mut r, mut c) = (0, 0);
    let (mut dr, mut dc) = (usize::MAX, 0);
    for _ in 0..n {
        if grid[r][c] == '.' {
            grid[r][c] = '#';
            (dr, dc) = rot(dr, dc);
        } else {
            grid[r][c] = '.';
            (dr, dc) = unti_rot(dr, dc);
        }
        r = (r + h).wrapping_add(dr) % h;
        c = (c + w).wrapping_add(dc) % w;
    }

    for v in grid {
        println!("{}", v.iter().collect::<String>())
    }
}
