use proconio::*;

fn bin_search(p: usize, a: &[usize]) -> usize {
    let n = a.len();
    let (mut l, mut r) = (-1, n as i32);
    while r - l > 1 {
        let m = (r + l) / 2;
        if a[m as usize] <= p {
            l = m;
        } else {
            r = m;
        }
    }
    r as usize
}

fn main() {
    input! {n: usize, a: [usize; n], q: usize, q: [(usize, usize); q]}

    let mut cum = vec![0; n];
    for i in 1..n {
        cum[i] += cum[i - 1];
        if i % 2 == 0 {
            cum[i] += a[i] - a[i - 1];
        }
    }
    // eprintln!("cum: {:?}", cum);

    for (l, r) in q {
        let li = bin_search(l, &a);
        let ri = bin_search(r, &a);
        if ri == 0 {
            continue;
        }
        let ri = ri - 1;
        let mut res = cum[ri] - if li == 0 { 0 } else { cum[li] };
        // eprintln!("li: {}, ri: {}, res: {:?}", li, ri, res);
        if a[ri] < r && ri % 2 == 1 {
            res += r - a[ri];
        }
        if l < a[li] && li % 2 == 0 {
            res += a[li] - l;
        }

        println!("{}", res)
    }
}
