use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [marker::Bytes; n]}

    let mut buf = vec![];
    for i in 0..n {
        for j in 0..n {
            if i + m > n || j + m > n {
                continue;
            }

            let mut b = vec![];
            for k in i..i + m {
                for l in j..j + m {
                    b.push(s[k][l]);
                }
            }
            buf.push(b);
        }
    }

    buf.sort_unstable();
    buf.dedup();
    println!("{}", buf.len())
}
