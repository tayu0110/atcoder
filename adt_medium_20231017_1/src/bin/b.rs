use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut map = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {k: usize, x: [usize; k]}

        for i in 0..k {
            for j in 0..k {
                map[x[i] - 1][x[j] - 1] = true;
            }
        }
    }

    if map.into_iter().flatten().fold(true, |s, v| s & v) {
        println!("Yes")
    } else {
        println!("No")
    }
}
