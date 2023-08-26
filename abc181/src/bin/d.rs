use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    let s = s
        .into_iter()
        .map(|c| (c - b'0') as usize)
        .collect::<Vec<_>>();
    if s.len() == 1 {
        if s[0] % 8 == 0 {
            println!("Yes")
        } else {
            println!("No")
        }
        return;
    } else if s.len() == 2 {
        if (s[0] * 10 + s[1]) % 8 == 0 {
            println!("Yes")
        } else if (s[1] * 10 + s[0]) % 8 == 0 {
            println!("Yes")
        } else {
            println!("No")
        }
        return;
    }

    let mut v = vec![0; 10];

    for c in s {
        v[c] += 1;
    }

    for i in 0..10 {
        if v[i] == 0 {
            continue;
        }
        v[i] -= 1;
        for j in 0..10 {
            if v[j] == 0 {
                continue;
            }
            v[j] -= 1;
            for k in 0..10 {
                if v[k] == 0 {
                    continue;
                }
                if (i * 100 + j * 10 + k) % 8 == 0 {
                    println!("Yes");
                    return;
                }
            }
            v[j] += 1;
        }
        v[i] += 1;
    }

    println!("No")
}
