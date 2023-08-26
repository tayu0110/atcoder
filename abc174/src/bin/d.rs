use proconio::*;

fn main() {
    input! {n: usize, mut s: marker::Chars}

    let mut res = 0;
    let (mut l, mut r) = (0, n - 1);
    while l < r {
        while l < n && s[l] == 'R' {
            l += 1;
        }
        while s[r] == 'W' {
            if r == 0 {
                break;
            }
            r -= 1;
        }

        if l < r {
            res += 1;
            s.swap(l, r);
        }
    }

    println!("{}", res)
}
