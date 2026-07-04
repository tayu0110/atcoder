use proconio::*;

const M: usize = 100;

fn solve(
    level: usize,
    l: usize,
    r: usize,
    scnt: &[usize],
    ccnt: &[[usize; 26]],
    s: [&[u8]; 2],
    c: usize,
) -> usize {
    if level <= 2 {
        return s[level - 1][l - 1..r]
            .iter()
            .filter(|&&b| (b - b'a') as usize == c)
            .count();
    }

    if l == 1 && scnt[level] == r {
        return ccnt[level][c];
    }

    let bound = scnt[level - 1];
    if r <= bound {
        return solve(level - 1, l, r, scnt, ccnt, s, c);
    }
    if bound < l {
        return solve(level - 2, l - bound, r - bound, scnt, ccnt, s, c);
    }

    solve(level - 1, l, bound, scnt, ccnt, s, c) + solve(level - 2, 1, r - bound, scnt, ccnt, s, c)
}

fn main() {
    input! {x: marker::Bytes, y: marker::Bytes, q: usize}

    let mut scnt = [0usize; M];
    let mut ccnt = [[0usize; 26]; M];
    scnt[1] = x.len();
    for &c in &x {
        ccnt[1][(c - b'a') as usize] += 1;
    }
    scnt[2] = y.len();
    for &c in &y {
        ccnt[2][(c - b'a') as usize] += 1;
    }

    let mut max = 0;
    for i in 3..M {
        scnt[i] = scnt[i - 1] + scnt[i - 2];
        for j in 0..26 {
            ccnt[i][j] = ccnt[i - 1][j] + ccnt[i - 2][j];
        }
        if scnt[i] > 1usize << 60 {
            max = i;
            break;
        }
    }

    for _ in 0..q {
        input! {l: usize, r: usize, c: char}
        println!(
            "{}",
            solve(max, l, r, &scnt, &ccnt, [&x, &y], (c as u8 - b'a') as usize)
        );
    }
}
