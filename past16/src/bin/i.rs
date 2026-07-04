use cpio::*;

fn main() {
    scan!(n: usize, m: usize, k: usize, mut a: [usize; n]);

    let (mut l, mut r) = (*a.iter().min().unwrap(), 1usize << 57);
    while r - l > 1 {
        let mid = (r + l) / 2;
        let cnt = a
            .iter()
            .filter_map(|&a| (mid > a).then(|| (mid - a + k - 1) / k))
            .sum::<usize>();

        if cnt <= m {
            l = mid;
        } else {
            r = mid;
        }
    }

    let mut cnt = 0;
    for i in 0..n {
        if l > a[i] {
            let c = (l - a[i] + k - 1) / k;
            cnt += c;
            a[i] += c * k;
        }
    }

    if cnt < m {
        assert!(m - cnt <= n);
        let mut index = (0..n).collect::<Vec<_>>();
        index.sort_unstable_by_key(|&i| (a[i], i));
        for &i in &index[..m - cnt] {
            a[i] += k;
        }
    }

    putln!(a, @sep = " ");
}
