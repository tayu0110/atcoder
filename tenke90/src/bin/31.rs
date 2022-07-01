use proconio::input;

fn main() {
    input! {n: usize, w: [usize; n], b: [usize; n]};

    const W: usize = 51;
    const B: usize = W * W + 51;

    let mut grundy = vec![vec![0; B]; W];
    for nw in 0..W {
        for nb in 0..B {
            if nw == 0 && nb < 2 {
                continue;
            }
            let mut buf = vec![];
            if nb >= 2 {
                for k in 1..=nb/2 {
                    buf.push(grundy[nw][nb - k]);
                }
            }
            if nw > 0 && nb + nw < B {
                buf.push(grundy[nw - 1][nb + nw]);
            }
            buf.sort();
            // eprintln!("buf: {:?}", buf);
            let mut mex = match buf.last() {
                Some(mex) => *mex,
                _ => {
                    eprintln!("nw: {}, nb: {}", nw, nb);
                    panic!("oh my god...");
                }
            } + 1;
            for (i, v) in buf.iter().enumerate() {
                if i+1 < buf.len() && buf[i+1] - *v > 1 {
                    mex = *v + 1;
                    break;
                }
            }
            if buf[0] != 0 {
                mex = 0;
            }
            grundy[nw][nb] = mex;
        }
    }

    let mut x = 0;
    for (s, t) in w.iter().zip(b) {
        let s = *s;
        x ^= grundy[s][t];
    }

    if x == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}