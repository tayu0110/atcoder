use proconio::*;

fn main() {
    input! {q: usize}

    let mut t = vec![];
    for _ in 0..q {
        input! {ty: usize, k: usize}

        if ty == 1 {
            t.push(k);
        } else {
            println!("{}", t[t.len() - k])
        }
    }
}
