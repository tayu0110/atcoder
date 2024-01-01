use proconio::*;

const MOD: u32 = 1000_000_007;

const fn mul<const L: usize, const M: usize, const N: usize>(
    a: [[u32; M]; L],
    b: [[u32; N]; M],
) -> [[u32; N]; L] {
    let mut res = [[0; N]; L];
    let mut i = 0;
    while i < L {
        let mut j = 0;
        while j < N {
            let mut k = 0;
            while k < M {
                res[i][j] += (a[i][k] as u64 * b[k][j] as u64 % MOD as u64) as u32;
                res[i][j] %= MOD;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    res
}

const fn init<const K: usize>() -> [[u32; K]; K] {
    assert!(1 << K.trailing_zeros() == K);
    let mut res = [[0; K]; K];
    let mut i = 0;
    let nk = K.trailing_zeros();
    while i < K {
        let mut j = 0;
        let t = i ^ (K - 1);
        'B: while j < K >> 1 {
            let mut k = 0;
            let mut t = t;
            while k < nk - 1 {
                if j & (1 << k) != 0 {
                    if t & (1 << k) != 0 || t & (1 << (k + 1)) != 0 {
                        j += 1;
                        continue 'B;
                    }
                    t |= 1 << k;
                    t |= 1 << (k + 1);
                }
                k += 1;
            }
            res[i][t] = 1;
            j += 1;
        }
        i += 1;
    }
    res
}

fn pow<const N: usize>(a: [[u32; N]; N], exp: usize) -> [[u32; N]; N] {
    if exp == 0 {
        let mut a = [[0; N]; N];
        for i in 0..N {
            a[i][i] = 1;
        }
        a
    } else if exp == 1 {
        a
    } else {
        let r = pow(a, exp >> 1);
        let mut r = mul(r, r);
        if exp & 1 != 0 {
            r = mul(r, a);
        }
        r
    }
}

fn solve<const K: usize>(n: usize) -> u32 {
    let mut t = [[0; 1]; K];
    t[K - 1][0] = 1;
    let res = mul(pow(init::<K>(), n), t);
    res[K - 1][0]
}

fn main() {
    input! {k: usize, n: usize}

    println!(
        "{}",
        if k == 2 {
            solve::<{ 1 << 2 }>(n)
        } else if k == 3 {
            solve::<{ 1 << 3 }>(n)
        } else {
            solve::<{ 1 << 4 }>(n)
        }
    );
}
