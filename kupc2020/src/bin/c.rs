use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn main() {
    const N: usize = 13;
    let mut rng = thread_rng();
    'base: loop {
        let mut t = vec![];
        let mut set = HashSet::new();
        for _ in 0..N {
            'inner: loop {
                let c = (0..N)
                    .map(|_| (rng.gen_range::<u8, u8, u8>(0, 26) + b'a') as char)
                    .collect::<Vec<_>>();
                for j in 0..N - 1 {
                    let s = format!("{}{}", c[j], c[j + 1]);
                    if set.contains(&s) {
                        continue 'inner;
                    }
                    set.insert(s);
                }
                t.push(c);
                break;
            }
        }
        eprintln!("t: {:?}", t);

        for i in 0..N {
            for j in 0..N - 1 {
                let s = format!("{}{}", t[i][j], t[i][j + 1]);
                if set.contains(&s) {
                    continue 'base;
                }
                set.insert(s);
            }
        }

        for v in t {
            println!("{}", v.into_iter().collect::<String>())
        }
        return;
    }
}
