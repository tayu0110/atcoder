use proconio::*;

fn main() {
    input! {a: usize, b: usize, t: usize}

    let d = b - a;
    for now in (a..).step_by(d) {
        if now >= t {
            println!("{}", now);
            return;
        }
    }
}
