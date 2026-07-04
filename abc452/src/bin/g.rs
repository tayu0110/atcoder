use proconio::*;

fn solve2(rle: &[(usize, usize)]) -> usize {
    if rle.is_empty() {
        return 0;
    }

    if rle.len() == 1 {
        let (p, cnt) = rle[0];
        return cnt - p + 1;
    }

    let mut ret = 0;
    let n = rle.len();
    if rle[0].0 < rle[0].1 {
        let (p, cnt) = rle[0];
        ret += cnt - p;
    }
    // if rle[n - 1].0 < rle[n - 1].1 {
    //     let (p, cnt) = rle[n - 1];
    //     ret += cnt - p;
    // }
    ret += n * (n + 1) / 2;
    eprintln!("solve2: {ret}");
    ret
}

fn solve(rle: &[(usize, usize)]) -> usize {
    if rle.is_empty() {
        return 0;
    }

    let mut ret = 0;
    let mut l = 0;
    for i in 1..rle.len() {
        if rle[i].0 < rle[i].1 {
            eprintln!("solve: l: {l}, i: {i}, ret: {ret}");
            ret += solve2(&rle[l..=i]);
            l = i;
        }
    }

    ret + solve2(&rle[l..])
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut rle = vec![];
    for a in a {
        match rle.last_mut() {
            Some((p, cnt)) if *p == a => *cnt += 1,
            _ => rle.push((a, 1)),
        }
    }
    eprintln!("rle: {rle:?}");

    let mut ret = 0;
    let mut l = 0;
    for i in 0..rle.len() {
        if rle[i].0 > rle[i].1 {
            ret += solve(&rle[l..i]);
            eprintln!("l: {l}, i: {i}, ret: {ret}");
            l = i + 1;
        }
    }

    if l < rle.len() {
        ret += solve(&rle[l..]);
        eprintln!("l: {l}, ret: {ret}");
    }

    println!("{ret}");
}
