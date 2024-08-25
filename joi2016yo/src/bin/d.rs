use proconio::*;

fn main() {
    input! {n: usize, t: i64, q: i64, p: [(i64, u8); n], query: [usize; q]}

    let mut stack = vec![];
    let mut pos = vec![0; n];
    let mut s = i64::MIN;
    for (i, (a, d)) in p.into_iter().enumerate() {
        if d == 1 {
            stack.push((i, a));
            s = i64::MIN;
        } else {
            if let Some((j, pa)) = stack.pop() {
                s = (pa + a) / 2;
                if a - s <= t {
                    pos[i] = s;
                    pos[j] = s;
                } else {
                    pos[i] = a - t;
                    pos[j] = pa + t;
                }

                while let Some((j, pa)) = stack.pop() {
                    if s - pa <= t {
                        pos[j] = s;
                    } else {
                        pos[j] = pa + t;
                    }
                }
            } else {
                if a - s <= t {
                    pos[i] = s;
                } else {
                    pos[i] = a - t;
                }
            }
        }
    }

    while let Some((i, a)) = stack.pop() {
        pos[i] = a + t;
    }

    for query in query {
        println!("{}", pos[query - 1]);
    }
}
