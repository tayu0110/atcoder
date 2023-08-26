use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut r: usize}

    let mut f = vec![];
    let mut b = vec![];
    let mut rem = n - k;
    for i in 1..=n {
        if rem <= r {
            r -= rem;
            b.push(i);
        } else {
            f.push(i);
        }

        rem = rem.saturating_sub(1);
    }

    if r > 0 {
        println!("No Luck");
        return;
    }

    f.extend(b.iter().rev());
    println!("{}", f.iter().join(" "))
}
