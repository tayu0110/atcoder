use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut c = vec![0; n];
    for _ in 0..q {
        input! {t: usize, x: usize}

        if t == 1 {
            c[x - 1] += 1;
        } else if t == 2 {
            c[x - 1] += 2;
        } else {
            if c[x - 1] >= 2 {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}
