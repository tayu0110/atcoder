fn pow_mod(a: u64, n: u64, p: u64) -> u64 {
    if n == 0 {
        1
    } else if n == 1 {
        a % p
    } else {
        let res = pow_mod(a, n / 2, p);
        if n % 2 == 1 {
            res * res % p * a % p
        } else {
            res * res % p
        }
    }
}

const R: u64 = 998244353;
const N: u64 = 1000_000_007;

fn mr(res: char, t: char, t_val: u64, n: u64, ninv: u64, rinv: u64) -> u64 {
    // MR(T) = (T + (T*NINV mod R) * N) * RINV
    // res = MR(t)
    //      D = t mo R          rem D {t}
    //      E = D * NINV        mul E D {NINV}
    //      E = E mod R         rem E E
    //      F = E * N           mul F E {N}
    //      T = t + F           add T {t} F
    //      res = T / R         mul {res} T {RINV}
    let d = t_val % R;
    println!("rem D {}", t);
    let e = d.wrapping_mul(ninv);
    println!("mul E D {}", ninv);
    let e = e % R;
    println!("rem E E");
    let f = e.wrapping_mul(n);
    println!("mul F E {}", n);
    let nt = t_val + f;
    println!("add T {} F", t);
    println!("mul {} T {}", res, rinv);
    nt.wrapping_mul(rinv)
}

fn main() {
    let (a, b) = (0u64, 543264352u64);
    let expected_res = a * b % N;

    // R^{-1} mod 2^64
    const R_INV: u64 = {
        let inv = R.wrapping_mul(2u64.wrapping_sub(R.wrapping_mul(R)));
        let inv = inv.wrapping_mul(2u64.wrapping_sub(R.wrapping_mul(inv)));
        let inv = inv.wrapping_mul(2u64.wrapping_sub(R.wrapping_mul(inv)));
        let inv = inv.wrapping_mul(2u64.wrapping_sub(R.wrapping_mul(inv)));
        inv.wrapping_mul(2u64.wrapping_sub(R.wrapping_mul(inv)))
    };
    assert_eq!(R.wrapping_mul(R_INV), 1);

    const R2: u64 = R * R % N;

    // N * NINV = -1 mod R
    let ninv = pow_mod(N, R - 2, R);
    assert_eq!(N.wrapping_mul(ninv) % R, 1);
    let ninv = (R - 1).wrapping_mul(ninv) % R;
    assert_eq!(N.wrapping_mul(ninv) % R, R - 1);

    println!("40");
    // A = A * R2
    let a = a.wrapping_mul(R2);
    println!("mul A A {}", R2);
    let a = mr('A', 'A', a, N, ninv, R_INV);

    // B = B * R2;
    let b = b.wrapping_mul(R2);
    println!("mul B B {}", R2);
    let b = mr('B', 'B', b, N, ninv, R_INV);

    // C = A * B
    let c = a.wrapping_mul(b);
    println!("mul C A B");
    let c = mr('C', 'C', c, N, ninv, R_INV);
    let c = mr('C', 'C', c, N, ninv, R_INV);

    eprintln!("incomplete response: {}", c);

    // C = C * R2
    let c = c.wrapping_mul(R2);
    println!("mul C C {}", R2);
    let c = mr('C', 'C', c, N, ninv, R_INV);
    let c = mr('C', 'C', c, N, ninv, R_INV);
    eprintln!("final response: {}", c);
    assert_eq!(c, expected_res);
}
