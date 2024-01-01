use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: usize, mut a: [usize; n], mut b: [usize; m]}
    a.sort_unstable();
    b.sort_unstable();

    let mut sum = b.iter().sum::<usize>();

    let mut res = 0usize;
    let mut now = m as i32 - 1;
    for a in a {
        while now >= 0 && a + b[now as usize] > p {
            sum -= b[now as usize];
            now -= 1;
        }
        res += (m - (now + 1) as usize) * p;
        res += a * (now + 1) as usize + sum;
    }

    println!("{res}")
}
