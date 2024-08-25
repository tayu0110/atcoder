use proconio::*;

fn main() {
    input! {s: marker::Chars}
    let len = s.len();

    for i in 0..len {
        if s[i] == 'B' {
            for j in i + 1..len {
                if s[j] == 'B' && i % 2 == j % 2 {
                    println!("No");
                    return;
                }
            }
        }
    }

    let mut f = false;
    for i in 0..len {
        if s[i] == 'R' {
            f = true;
            for j in i + 1..len {
                if s[j] == 'R' {
                    println!("No");
                    return;
                }
                if s[j] == 'K' {
                    break;
                }
            }
        }

        if s[i] == 'K' && !f {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
