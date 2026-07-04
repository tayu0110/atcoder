use proconio::*;

fn main() {
    input! {n: usize, t: usize, a: [usize; n]}

    let mut ret = 0;
    let mut opened = 0;
    for a in a {
        if opened < a {
            ret += a - opened;
            opened = a + 100;
        }
    }
    if opened < t {
        ret += t - opened;
    }

    println!("{ret}")
}
