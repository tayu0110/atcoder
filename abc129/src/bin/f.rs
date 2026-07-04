use math::MathInt;
use proconio::*;

fn mul(a: [[usize; 3]; 3], b: [[usize; 3]; 3], m: usize) -> [[usize; 3]; 3] {
    let mut res = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                res[i][j] += a[i][k] * b[k][j];
                res[i][j] %= m;
            }
        }
    }
    res
}

fn pow(a: [[usize; 3]; 3], mut n: usize, m: usize) -> [[usize; 3]; 3] {
    let mut res = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    let mut val = a;
    while n > 0 {
        if n & 1 != 0 {
            res = mul(val, res, m);
        }
        val = mul(val, val, m);
        n >>= 1;
    }
    res
}

fn main() {
    input! {l: usize, a: usize, b: usize, m: usize}

    let mut res = 0;
    let mut seen = 0;
    for d in 1..19 {
        let (mut lh, mut rh) = (seen, l);
        while rh - lh > 1 {
            let mid = (rh + lh) / 2;
            let s = a + b * mid;
            if s.ilog10() + 1 <= d {
                lh = mid;
            } else {
                rh = mid;
            }
        }

        if (a + b * lh).ilog10() + 1 > d {
            continue;
        }

        let cnt = rh - seen;
        if cnt == 0 {
            continue;
        }

        let mat = [
            [10usize.pow_mod(d as u64, m), 0, 0],
            [1, 1, 0],
            [0, b % m, 1],
        ];
        let mat = pow(mat, cnt, m);
        res = res * mat[0][0] % m + (a + b * seen) % m * mat[1][0] % m + mat[2][0];
        res %= m;
        assert_eq!(
            (a + b * rh) % m,
            (res * mat[0][1] % m + (a + b * seen) % m * mat[1][1] % m + mat[2][1]) % m
        );

        seen = rh;
    }

    assert_eq!(seen, l);

    println!("{res}")
}
