use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut s: marker::Bytes}

        for i in 0..n - 1 {
            if s[i] > s[i + 1] {
                let c = s.remove(i);
                for j in i..s.len() {
                    if c < s[j] {
                        s.insert(j, c);
                        break;
                    }
                }

                if s.len() != n {
                    s.push(c);
                }
                break;
            }
        }

        println!("{}", s.iter().map(|&c| c as char).collect::<String>())
    }
}
