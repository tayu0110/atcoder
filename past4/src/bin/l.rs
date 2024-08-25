use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut h: [usize; n]}

    let (mut o, mut e) = (0, 0);
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {v: usize}
            o += v;
        } else if ty == 2 {
            input! {v: usize}
            e += v;
        } else {
            input! {u: usize, v: usize}
            h[u - 1] += v;
        }
    }

    h.push(usize::MAX);
    for v in h.chunks_exact_mut(2) {
        v[0] += o;
        v[1] = v[1].saturating_add(e);
    }

    h.pop();

    println!("{}", h.windows(2).filter(|v| v[0] == v[1]).count())
}
