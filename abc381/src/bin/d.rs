use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    for start in 0..2 {
        let mut cnt = vec![0; n + 1];
        let (mut l, mut r) = (start, start);
        while l + 1 < n {
            if a[l] != a[l + 1] {
                l += 2;
                r = r.max(l);
                continue;
            }

            while r + 1 < n && a[r] == a[r + 1] && cnt[a[r]] == 0 {
                cnt[a[r]] += 1;
                r += 2;
            }

            res = res.max(r - l);
            if l != r {
                cnt[a[l]] -= 1;
            } else {
                r += 2;
            }
            l += 2;
        }
    }

    println!("{res}")
}
