use proconio::*;
use static_modint::{combination, Mod998244353};

const M: usize = 998244353;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut both = vec![0i32; n + 1];
    let mut either = vec![0i32; n + 1];
    for (l, r) in p {
        let ml = n - r;
        let mr = n - l;
        if l <= mr && mr <= r {
            both[l] += 1;
            both[mr + 1] -= 1;
            either[mr + 1] += 1;
            either[r + 1] -= 1;
        } else if l <= ml && ml <= r {
            either[l] += 1;
            either[ml] -= 1;
            both[ml] += 1;
            both[r + 1] -= 1;
        } else {
            either[l] += 1;
            either[r + 1] -= 1;
        }
    }

    for i in 0..n {
        both[i + 1] += both[i];
        either[i + 1] += either[i];
    }
    assert!(both.iter().all(|&b| b >= 0));
    assert!(either.iter().all(|&b| b >= 0));
    // eprintln!("both: {both:?}, either: {either:?}");

    let com = combination::<Mod998244353>(n as u32 + 10);
    let mut ret = 0;
    for i in 1..n {
        // assert!(both[i] == both[n - i]);
        if either[i] + either[n - i] + both[i] != n as i32 {
            continue;
        }
        if either[i] > i as i32 || either[n - i] > (n - i) as i32 {
            continue;
        }
        let diff = i - either[i] as usize;
        ret += com(both[i] as u32, diff as u32).val() as usize;
        ret %= M;
    }

    println!("{}", ret);
}
