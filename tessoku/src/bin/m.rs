use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut res = 0;
    for (i, &na) in a.iter().enumerate() {
        let (mut l, mut r) = (i, n);
        while r - l > 1 {
            let m = (r + l) / 2;
            if a[m] - na <= k {
                l = m;
            } else {
                r = m;
            }
        }

        res += r - i - 1;
    }

    println!("{}", res)
}
