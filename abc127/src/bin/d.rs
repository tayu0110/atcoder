use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], mut p: [(usize, usize); m]}

    p.sort_by_key(|&(l, r)| (r, l));

    let mut buf = vec![];
    while let Some((mut b, c)) = p.pop() {
        while b > 0 && buf.len() < n {
            buf.push(c);
            b -= 1;
        }
    }

    buf.extend(a);

    buf.sort();
    let len = buf.len();
    println!("{}", buf[len - n..len].iter().sum::<usize>())
}
