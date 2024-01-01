use proconio::*;

fn main() {
    input! {n: usize, k: usize, t: usize, mut a: [usize; k]}

    let mut cnt = vec![0; n];
    for a in a {
        cnt[a - 1] += 1;
    }

    let mut res = 0;
    let (mut l, mut r) = (0, 0);
    let mut f = false;
    while l < n {
        while r < n && r - l < t {
            if cnt[r] > 1 {
                res += cnt[r] - 1;
                cnt[r] = 1;
            }

            if cnt[r] == 1 {
                if f {
                    res += 1;
                    cnt[r] = 0;
                } else {
                    f = true;
                }
            }

            r += 1;
        }

        if cnt[l] == 1 {
            f = false;
        }

        l += 1;
    }

    println!("{res}")
}
