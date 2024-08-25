use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut p: [(usize, usize, char); n]}

    let m = 2 * k;
    p.iter_mut().for_each(|v| {
        v.0 %= m;
        v.1 %= m
    });

    let mut black = vec![vec![0u32; m + 1]; m + 1];
    let mut white = vec![vec![0u32; m + 1]; m + 1];
    for (x, y, c) in p {
        let t = if c == 'B' { &mut black } else { &mut white };
        t[x + 1][y + 1] += 1;
    }
    for i in 0..m + 1 {
        for j in 0..m {
            black[i][j + 1] += black[i][j];
            white[i][j + 1] += white[i][j];
        }
    }
    for i in 0..m {
        for j in 0..m + 1 {
            black[i + 1][j] += black[i][j];
            white[i + 1][j] += white[i][j];
        }
    }

    let get = |upper: usize, down: usize, left: usize, right: usize, buf: &Vec<Vec<u32>>| {
        (buf[upper][right] + buf[down][left])
            .checked_sub(buf[upper][left] + buf[down][right])
            .unwrap()
    };
    let calc = |i: usize, j: usize, b: &mut u32, black: &Vec<Vec<u32>>, white: &Vec<Vec<u32>>| {
        *b += get(i + k, i, j, j + k, black);
        *b += get(m, i + k, j + k, m, black);
        *b += get(i, 0, 0, j, black);
        *b += get(m, i + k, 0, j, black);
        *b += get(i, 0, j + k, m, black);

        *b += get(m, i + k, j, j + k, white);
        *b += get(i + k, i, j + k, m, white);
        *b += get(i, 0, j, j + k, white);
        *b += get(i + k, i, 0, j, white);
    };

    let mut res = 0;
    for i in 1..=k {
        for j in 1..=k {
            let mut b = 0;
            let mut w = 0;
            calc(i, j, &mut b, &black, &white);
            calc(i, j, &mut w, &white, &black);

            res = res.max(b).max(w);
        }
    }

    println!("{}", res);
}
