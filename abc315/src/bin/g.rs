use proconio::*;

// when ax + by = gcd(a, b), return (gcd(a, b), x, y)
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut s, mut t) = (a, b);
    // a(sx) + b(sy) = s    ... (i)
    let (mut sx, mut sy) = (1, 0);
    // a(tx) + b(ty) = t    ... (ii)
    let (mut tx, mut ty) = (0, 1);

    while t > 0 {
        let u = s / t;

        // (i) - u * (ii)
        sx -= u * tx;
        sy -= u * ty;
        s -= u * t;

        (s, sx, sy, t, tx, ty) = (t, tx, ty, s, sx, sy);
    }

    assert_eq!(a * sx + b * sy, s);

    (s, sx, sy)
}

fn main() {
    input! {n: i64, a: i64, b: i64, c: i64, x: i64}

    let x = x - a - b - c;
    if x < 0 {
        println!("0");
        return;
    }

    let (g, j, k) = ext_gcd(b, c);
    let (b, c) = (b / g, c / g);
    let mut res = 0;
    for i in 0..n {
        let x = x - a * i;
        if x < 0 {
            break;
        }

        if x % g != 0 {
            continue;
        }

        let x = x / g;
        let (j, k) = (j * x, k * x);
    }

    println!("{}", res)
}
