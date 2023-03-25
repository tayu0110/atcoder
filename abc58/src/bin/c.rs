use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Chars; n]}

    let mut t = vec![std::i32::MAX; 26];
    for s in s {
        let mut nt = vec![0; 26];
        for c in s {
            nt[c as usize - b'a' as usize] += 1;
        }

        t.iter_mut().zip(nt).for_each(|(t, nt)| *t = (*t).min(nt));
    }

    let mut res = String::new();
    for (i, c) in t.into_iter().enumerate() {
        res.extend((0..c).map(|_| (i as u8 + b'a') as char));
    }

    println!("{}", res)
}
