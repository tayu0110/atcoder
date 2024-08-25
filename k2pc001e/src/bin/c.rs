use proconio::*;

fn nth_element(n: usize) -> (usize, usize) {
    let (mut a, mut b) = (1, 1);
    for _ in 1..n {
        if a == 1 {
            a += b;
            b = 1;
        } else {
            a -= 1;
            b += 1;
        }
    }
    (a, b)
}

fn main() {
    input! {i: usize, j: usize}

    let (ni, mi) = nth_element(i);
    let (nj, mj) = nth_element(j);

    let (a, b) = (ni + nj, mi + mj);
    let k = a + b - 2;
    println!("{}", k * (k + 1) / 2 + b);
}
