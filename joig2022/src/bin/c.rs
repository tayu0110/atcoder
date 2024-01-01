use proconio::*;

fn main() {
    input! {n: usize}

    let mut agree = vec![0; n + 1];
    for i in 0..n {
        input! {x: usize, y: usize}
        agree[i + 1] = agree[i];

        if y <= agree[i] - agree[i - x] {
            agree[i + 1] += 1;
        }
    }

    println!("{}", agree[n])
}
