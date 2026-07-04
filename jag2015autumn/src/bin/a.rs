use proconio::*;

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes}

    let mut ti = 0;
    for &c in s.iter().skip(1).step_by(2) {
        while ti < t.len() && t[ti] != c {
            ti += 1;
        }

        if ti == t.len() {
            let mut ti = 0;
            for &c in s.iter().step_by(2) {
                while ti < t.len() && t[ti] != c {
                    ti += 1;
                }

                if ti == t.len() {
                    println!("No");
                    return;
                }
                ti += 1;
            }
            break;
        }
        ti += 1;
    }

    println!("Yes");
}
