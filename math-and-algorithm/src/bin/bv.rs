use proconio::*;

fn mul(a: [f64; 9], b: [f64; 9]) -> [f64; 9] {
    [
        a[0] * b[0] + a[1] * b[3] + a[2] * b[6],
        a[0] * b[1] + a[1] * b[4] + a[2] * b[7],
        a[0] * b[2] + a[1] * b[5] + a[2] * b[8],
        a[3] * b[0] + a[4] * b[3] + a[5] * b[6],
        a[3] * b[1] + a[4] * b[4] + a[5] * b[7],
        a[3] * b[2] + a[4] * b[5] + a[5] * b[8],
        a[6] * b[0] + a[7] * b[3] + a[8] * b[6],
        a[6] * b[1] + a[7] * b[4] + a[8] * b[7],
        a[6] * b[2] + a[7] * b[5] + a[8] * b[8],
    ]
}

fn mul3(a: [f64; 9], b: [f64; 3]) -> [f64; 3] {
    [
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2],
        a[3] * b[0] + a[4] * b[1] + a[5] * b[2],
        a[6] * b[0] + a[7] * b[1] + a[8] * b[2],
    ]
}

fn pow(a: [f64; 9], exp: usize) -> [f64; 9] {
    if exp == 0 {
        [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
    } else if exp == 1 {
        a
    } else {
        let r = pow(a, exp >> 1);
        let mut res = mul(r, r);
        if exp & 1 != 0 {
            res = mul(res, a);
        }
        res
    }
}

fn main() {
    input! {q: usize}

    let mut res = vec![];
    for _ in 0..q {
        input! {x: f64, y: f64, z: f64, t: usize}
        let r = pow([1. - x, y, 0., 0., 1. - y, z, x, 0., 1. - z], t);
        let r = mul3(r, [1., 1., 1.]);
        res.push(r);
    }

    for [a, b, c] in res {
        println!("{a} {b} {c}")
    }
}
