use proconio::*;

fn main() {
    input! {n: usize, ab: [(usize, usize); n], m: usize, s: [marker::Bytes; m]}

    let mut memo = [[[false; 26]; 10]; 11];
    for s in &s {
        for (i, c) in s.iter().enumerate() {
            memo[s.len()][i][(c - b'a') as usize] = true;
        }
    }

    for s in s {
        if s.len() != n {
            println!("No");
            continue;
        }
        let mut bad = false;
        for (&c, &(a, b)) in s.iter().zip(&ab) {
            bad |= !memo[a][b - 1][(c - b'a') as usize];
        }

        if !bad {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
