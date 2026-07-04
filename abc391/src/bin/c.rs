use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut count = vec![1; n + 1];
    let mut pos = vec![0; n + 1];
    for i in 1..=n {
        pos[i] = i;
    }

    let mut res = 0;
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {p: usize, h: usize}
            count[pos[p]] -= 1;
            if count[pos[p]] == 1 {
                res -= 1;
            }

            pos[p] = h;
            count[pos[p]] += 1;
            if count[pos[p]] == 2 {
                res += 1;
            }
        } else {
            println!("{}", res);
        }
    }
}
