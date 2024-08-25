use proconio::*;

fn main() {
    input! {s: marker::Chars, t: String}

    for i in 1..s.len() {
        for j in 0..i {
            let u = s
                .chunks(i)
                .filter(|v| v.len() > j)
                .map(|v| v[j])
                .collect::<String>();
            if u == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
