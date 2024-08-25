use proconio::*;

fn main() {
    input! {_: usize, k: usize, mut s: marker::Chars}

    while !s.is_empty() && s.last() != Some(&'I') {
        s.pop();
    }
    s.reverse();
    while !s.is_empty() && s.last() != Some(&'J') {
        s.pop();
    }
    s.reverse();
    let n = s.len();

    let (mut ji, mut oi, mut ii) = (0, 0, 0);
    let (mut j, mut o, mut i) = (0, 0, 0);
    let mut f = 0;
    let mut res = usize::MAX;
    while ji < n {
        while j < k && ji < n {
            if s[ji] == 'J' {
                j += 1;
            } else if ji < oi && s[ji] == 'O' {
                o -= 1;
            } else if ji < ii && s[ji] == 'I' {
                i -= 1;
            }
            ji += 1;
        }

        if oi < ji {
            o = 0;
            oi = ji;
        }

        while o < k && oi < n {
            if s[oi] == 'O' {
                o += 1;
            } else if oi < ii && s[oi] == 'I' {
                i -= 1;
            }
            oi += 1;
        }

        if ii < oi {
            i = 0;
            ii = oi;
        }

        while i < k && ii < n {
            if s[ii] == 'I' {
                i += 1;
            }
            ii += 1;
        }

        if j != k || o != k || i != k {
            break;
        }

        res = res.min(ii - f - 3 * k);

        f += 1;
        while f < n && s[f] != 'J' {
            f += 1;
        }
        j -= 1;
    }

    println!("{}", res as i64);
}
