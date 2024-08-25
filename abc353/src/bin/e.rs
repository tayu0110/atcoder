use proconio::*;

fn main() {
    input! {n: usize, mut s: [marker::Bytes; n]}

    s.sort_unstable();

    let mut res = 0;
    let mut buf = vec![(0, 0)];
    let mut cum = vec![0; n + 10];
    for i in 1..n {
        let diff = s[i - 1]
            .iter()
            .zip(s[i].iter())
            .take_while(|(s, t)| s == t)
            .count();

        let mut c = 1;
        while let Some((p, cnt)) = buf.pop() {
            if p < diff {
                buf.push((p, cnt));
                cum[buf.len()] = cum[buf.len() - 1] + diff * c;
                buf.push((diff, c));
                break;
            } else {
                c += cnt;
            }
        }

        if buf.is_empty() {
            buf.push((0, 0));
            cum[0] = 0;
        }

        res += cum[buf.len() - 1];
    }

    println!("{res}")
}
