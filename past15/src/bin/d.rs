use proconio::*;

fn main() {
    input! {mut a: u16, b: u16, c: u16, d: u16, r: u16}

    let mut f = vec![false; c as usize + r as usize + 1];
    for now in (0..=c + r).step_by(d.into()) {
        if now >= b {
            a = c;
        }
        if now < a + r {
            for a in a.max(now)..(a + r).min(c + r + 1) {
                f[a as usize] = true;
            }
        }
    }

    println!(
        "{}",
        if (c..c + r).all(|g| f[g as usize]) {
            "Yes"
        } else {
            "No"
        }
    )
}
