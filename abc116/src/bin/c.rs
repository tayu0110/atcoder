use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n]}

    let mut res = 0;
    for i in 0.. {
        let mut f = false;
        let mut g = false;
        for &h in &h {
            if h > i {
                if !f {
                    f = true;
                    res += 1;
                }
            } else {
                f = false;
            }

            g |= f;
        }

        if !g {
            break;
        }
    }

    println!("{}", res)
}
