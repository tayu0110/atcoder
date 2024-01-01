use proconio::*;

fn main() {
    input! {n: usize, mut s: marker::Chars}
    s.sort();

    let mut res = 0;
    let p = 10usize.pow(n as u32 - 1);
    eprintln!("p: {p}");
    for i in 0..1000_000_0 {
        let q = i * i;
        if q < p * 10 {
            let mut q = q.to_string().chars().collect::<Vec<_>>();
            if q.len() < s.len() {
                q.resize(n, '0');
            }
            q.sort();

            if q == s {
                res += 1;
            }
        }
    }

    println!("{res}")
}
