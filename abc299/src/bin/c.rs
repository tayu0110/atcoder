use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    if s.iter().all(|&c| c == 'o') || s.iter().all(|&c| c != 'o') {
        println!("-1");
        return;
    }

    let mut i = 0;
    let mut res = 0;
    while i < n {
        if s[i] == 'o' {
            let mut j = i;
            while j < n && s[j] == 'o' {
                j += 1;
            }
            res = res.max(j - i);
            i = j;
        }
        i += 1;
    }

    println!("{}", res)
}
