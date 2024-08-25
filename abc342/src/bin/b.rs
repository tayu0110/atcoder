use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n], q: usize}

    let mut pos = vec![0; n + 1];
    for (i, p) in p.into_iter().enumerate() {
        pos[p] = i;
    }

    for _ in 0..q {
        input! {a: usize, b: usize}

        if pos[a] < pos[b] {
            println!("{a}")
        } else {
            println!("{b}")
        }
    }
}
