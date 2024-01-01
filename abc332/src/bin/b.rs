use proconio::*;

fn main() {
    input! {k: usize, g: usize, m: usize}

    let mut ng = 0;
    let mut nm = 0;
    for _ in 0..k {
        if g == ng {
            ng = 0;
        } else if nm == 0 {
            nm = m;
        } else {
            let diff = nm.min(g - ng);
            ng += diff;
            nm -= diff;
        }
    }

    println!("{ng} {nm}")
}
