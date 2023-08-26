use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut b: [[usize; m]; n]}

    if b.windows(2)
        .any(|v| !v[0].iter().zip(&v[1]).all(|(&l, &r)| l < r && r - l == 7))
    {
        println!("No");
        return;
    }

    let mut k = b.swap_remove(0);
    if k.windows(2).any(|v| v[0] > v[1] || v[1] - v[0] != 1) {
        println!("No");
        return;
    }
    k.iter_mut().for_each(|v| {
        *v %= 7;
        if *v == 0 {
            *v += 7
        }
    });

    if &k[0] <= k.last().unwrap() {
        println!("Yes")
    } else {
        println!("No")
    }
}
