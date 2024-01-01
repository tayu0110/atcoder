use proconio::*;

fn main() {
    input! {a: u64, mut b: u64, c: u64}

    if c == 1 {
        println!("No");
        return;
    }

    if b > 60 {
        println!("Yes");
        return;
    }

    let mut nc = 1u64;
    let mut val = c;
    while b > 0 {
        if b & 1 != 0 {
            nc = nc.saturating_mul(val);
        }
        val = val.saturating_mul(val);
        b >>= 1;
    }

    if a < nc {
        println!("Yes")
    } else {
        println!("No")
    }
}
