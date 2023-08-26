use proconio::*;

fn main() {
    input! {n: usize, mut k: usize, h: [usize; n]}

    let mut res = 0;
    let (mut l, mut r) = (0, 0);
    while l < n {
        while r < n {
            if h[r] == 1 {
                r += 1;
                continue;
            }

            if k == 0 {
                break;
            }

            r += 1;
            k -= 1;
        }

        res = res.max(r - l);

        if h[l] == 0 {
            k += 1;
        }

        l += 1;
    }

    println!("{}", res)
}
