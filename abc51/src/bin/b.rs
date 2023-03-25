use proconio::*;

fn main() {
    input! {k: usize, s: usize}

    let mut res = 0;
    for x in 0..=k {
        for y in 0..=k {
            let z = s.saturating_sub(x + y);
            if z <= k && x + y + z == s {
                res += 1;
            }
        }
    }

    println!("{}", res)
}
