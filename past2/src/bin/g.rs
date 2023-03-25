use proconio::input;

fn main() {
    input! {q: usize}

    let mut nt = std::collections::VecDeque::new();
    for _ in 0..q {
        input! {t: usize}
        if t == 1 {
            input! {c: char, x: usize}
            nt.push_back((c, x));
        } else {
            input! {mut d: usize}
            let mut res = vec![0; 26];
            while let Some((c, x)) = nt.pop_front() {
                if d >= x {
                    d -= x;
                    res[c as usize - b'a' as usize] += x;
                } else {
                    res[(c as u8 - b'a') as usize] += d;
                    nt.push_front((c, x - d));
                    break;
                }
            }

            println!("{}", res.into_iter().map(|v| v * v).sum::<usize>())
        }
    }
}
