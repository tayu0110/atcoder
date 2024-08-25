use iolib::*;

fn main() {
    scan!(h: usize, w: usize, m: usize, query: [(u8, u32, u32); m]);

    let mut rows = h;
    let mut cols = w;
    let mut rset = vec![false; h + 1];
    let mut cset = vec![false; w + 1];

    let mut res = vec![0; 200010];
    for (t, a, x) in query.into_iter().rev() {
        if t == 1 {
            if !rset[a as usize] {
                res[x as usize] += cols;
                rset[a as usize] = true;
                rows -= 1;
            }
        } else {
            if !cset[a as usize] {
                res[x as usize] += rows;
                cset[a as usize] = true;
                cols -= 1;
            }
        }
    }

    res[0] += rows * cols;

    let res = res
        .into_iter()
        .enumerate()
        .filter(|&(_, c)| c > 0)
        .collect::<Vec<_>>();
    putln!(res.len());
    for (color, number) in res {
        putitln!([color, number].into_iter(), sep = ' ');
    }
}
