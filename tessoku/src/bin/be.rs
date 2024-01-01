use proconio::*;

#[fastout]
fn main() {
    input! {n: usize, q: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|v| *v -= 1);

    let mut d = vec![vec![usize::MAX; 31]; n];
    for (i, &a) in a.iter().enumerate() {
        d[i][0] = a;
    }

    for i in 1..31 {
        for j in 0..n {
            d[j][i] = d[d[j][i - 1]][i - 1];
        }
    }

    for _ in 0..q {
        input! {mut x: usize, mut y: usize}
        x -= 1;

        for i in (0..31).rev() {
            if 1 << i <= y {
                x = d[x][i];
                y ^= 1 << i;
            }
        }

        println!("{}", x + 1);
    }
}
