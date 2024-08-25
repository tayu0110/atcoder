use proconio::input;

fn main() {
    input! {n: usize, s: [String; n]}
    const MAX: usize = 52 * 52 * 52;
    let to_index = |c: &[u8]| c.iter().fold(0, |s, c| s * 52 + if c <= &b'Z' { *c - b'A' } else { *c - b'a' + 26 } as usize );

    let mut f = vec![];
    let mut out = vec![0; MAX];
    let mut rt = vec![vec![]; MAX];
    // 0: unknown or draw, 1: win, 2: lose
    let mut k = vec![0; MAX];
    for s in s {
        let s = s.bytes().collect::<Vec<_>>();
        let len = s.len();
        let from = to_index(&s[0..3]);
        let to = to_index(&s[len-3..len]);

        f.push(to);
        out[from] += 1;
        rt[to].push(from);
    }

    let mut nt = std::collections::VecDeque::new();
    for i in 0..MAX {
        if out[i] == 0 {
            nt.push_back(i);
            k[i] = 2;
        }
    }

    while let Some(now) = nt.pop_front() {
        for to in &rt[now] {
            if k[*to] == 0 {
                out[*to] -= 1;
                if k[now] == 2 {
                    k[*to] = 1;
                    nt.push_back(*to);
                } else if out[*to] == 0 {
                    k[*to] = 2;
                    nt.push_back(*to);
                }
            }
        }
    }

    for r in f {
        if k[r] == 1 {
            println!("Aoki");
        } else if k[r] == 2 {
            println!("Takahashi");
        } else {
            println!("Draw");
        }
    }
}