use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let (mut l, mut r) = (0, 0);
    let mut res = 0;
    while l < n {
        let c = unsafe { *s.get_unchecked(l) };
        while r < n && unsafe { *s.get_unchecked(r) } == c {
            r += 1;
        }

        res += (n - r) * (r - l);
        l = r;
    }

    println!("{res}")
}
