use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], mut b: [usize; m]}
    a.sort();
    b.sort();

    let (mut l, mut r) = (0, 10000000000);
    while r - l > 1 {
        let x = (r + l) / 2;
        let mut na = 0;
        for &a in &a {
            if a <= x {
                na += 1;
            }
        }
        let mut nb = 0;
        for &b in &b {
            if x <= b {
                nb += 1;
            }
        }

        if na >= nb {
            r = x;
        } else {
            l = x;
        }
    }

    println!("{}", r)
}
