use itertools::Itertools;

use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn naive(n: usize, x: usize) -> usize {
    (0..n)
        .permutations(n)
        .filter(|v| {
            v.iter()
                .enumerate()
                .map(|(i, &v)| (i.max(v) - i.min(v)))
                .all(|v| v >= x)
        })
        .count()
}

fn x1(n: usize, memo: &mut Vec<Option<Modint>>) -> Modint {
    if let Some(res) = memo[n] {
        return res;
    }

    if n == 0 {
        return Modint::one();
    } else if n == 1 {
        return Modint::zero();
    } else if n == 2 {
        return Modint::one();
    }

    let nn = Modint::raw(n as u32);
    memo[n] = Some((nn - Modint::one()) * (x1(n - 1, memo) + x1(n - 2, memo)));
    memo[n].unwrap()
}

// fn x2(n: usize, memo: &mut Vec<Option<Modint>>) -> Modint {
//     if let Some(res) = memo[n] {
//         return res;
//     }

//     if n == 0 {
//         return Modint::one();
//     } else if n == 1 {
//         return Modint::zero();
//     } else if n == 2 {
//         return Modint::zero();
//     } else if n == 3 {
//         return Modint::zero();
//     } else if n == 4 {
//         return Modint::one();
//     } else if n == 5 {
//         return Modint::raw(4);
//     }

//     let nn = Modint::raw(n as u32);
//     let _one = Modint::one();
//     let _zero = Modint::zero();
//     memo[n] = Some((nn - Modint::one()) * (x1(n - 1, memo) + x1(n - 2, memo)));
//     memo[n].unwrap()
// }

fn main() {
    // input! {n: usize, x: usize}

    for x in 1..=5 {
        if x == 1 {
            let mut memo = vec![None; 10];
            x1(9, &mut memo);
            for n in 1..=10 {
                println!("n: {n:2}, x: {x}, {}", naive(n, x))
            }
        } else {
            for n in 1..10 {
                println!("n: {n:2}, x: {x}, {}", naive(n, x))
            }
        }
    }

    // let bitlen = 1 + 2 * (x - 1);
    // let mut dp = vec![vec![Modint::zero(); 1 << bitlen]; n + 1];
    // dp[0][0] = Modint::one();
    // for i in 0..n {
    //     for s in 0..1 << bitlen {
    //         if dp[i][s] == Modint::zero() {
    //             continue;
    //         }

    //         for j in 0..bitlen {
    //             if j < bitlen - 1 && i >= x && (s & (1 << (bitlen - 1))) == 0 {
    //                 continue;
    //             }

    //             if s & (1 << j) != 0 {
    //                 continue;
    //             }

    //             let next = s | (1 << j);
    //             eprintln!("next: {next:0b}");
    //             eprintln!("next_: {:0b}", (next << 1) & ((1 << bitlen) - 1));
    //             let now = dp[i][s];
    //             dp[i + 1][(next << 1) & ((1 << bitlen) - 1)] += now;
    //         }
    //     }
    // }
    // eprintln!("dp: {dp:?}");

    // let base = (1..=n)
    //     .map(|i| Modint::raw(i as u32))
    //     .fold(Modint::one(), |s, v| s * v);
    // eprintln!("base: {base:?}");

    // eprintln!("k: {:?}", ((1 << x) - 1) << x);
    // println!("{}", base - dp[n][((1 << (x - 1)) - 1) << x]);
}
