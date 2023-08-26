use proconio::*;

fn pow_mod(a: usize, n: usize, p: usize) -> usize {
    if a % p == 0 {
        0
    } else if n == 0 {
        1
    } else if n == 1 {
        a % p
    } else {
        let res = pow_mod(a, n / 2, p);
        if n % 2 == 0 {
            res * res % p
        } else {
            res * res % p * a % p
        }
    }
}

fn pow_mod_mat(a: usize, n: usize, p: usize) -> (usize, usize) {
    if n == 0 {
        (1, 0)
    } else if n == 1 {
        (a % p, 1)
    } else {
        let (ra, rb) = pow_mod_mat(a, n / 2, p);
        let (ra, rb) = (ra * ra % p, (ra * rb % p + rb) % p);
        if n % 2 == 1 {
            (ra * a % p, (a * rb % p + 1) % p)
        } else {
            (ra, rb)
        }
    }
}

fn main() {
    input! {n: usize, p: [(usize, usize); n], b: usize}

    let mut now = 0;
    for (a, l) in p {
        let len = a.to_string().len();
        let t = pow_mod(10, len, b);
        now *= pow_mod(t, l, b);
        now %= b;

        let (_, rb) = pow_mod_mat(t, l, b);

        now += a * rb % b;
        now %= b;
    }

    println!("{}", now)
}
