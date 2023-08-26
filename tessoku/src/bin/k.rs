use proconio::input;

fn main() {
    input! {n: usize, x: usize, a: [usize; n]}

    let (mut l, mut r) = (0, n);
    while r - l > 1 {
        let m = (r + l) / 2;
        if a[m] <= x {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", r)
}
