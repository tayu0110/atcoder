use itertools::Itertools;
use proconio::input;

fn get_size(
    now: usize,
    par: usize,
    used: &mut [bool],
    size: &mut [usize],
    t: &Vec<Vec<usize>>,
) -> usize {
    size[now] = 1;
    for &to in &t[now] {
        if used[to] {
            continue;
        }

        if to == par {
            continue;
        }

        size[now] += get_size(to, now, used, size, t);
    }

    size[now]
}

// ret: (min_size, node)
fn get_centroid(
    now: usize,
    par: usize,
    tree_size: usize,
    used: &mut [bool],
    size: &mut [usize],
    t: &Vec<Vec<usize>>,
) -> (usize, usize) {
    let mut res = (std::usize::MAX, 0);
    let mut max = tree_size - size[now];
    for &to in &t[now] {
        if used[to] {
            continue;
        }
        if to == par {
            continue;
        }

        res = res.min(get_centroid(to, now, tree_size, used, size, t));
        max = max.max(size[to]);
    }

    res.min((max, now))
}

fn decomposition(
    now: usize,
    used: &mut [bool],
    size: &mut [usize],
    res: &mut [i32],
    t: &Vec<Vec<usize>>,
) -> usize {
    let tree_size = get_size(now, now, used, size, t);
    let (_, centroid) = get_centroid(now, now, tree_size, used, size, t);

    used[centroid] = true;

    for &to in &t[centroid] {
        if used[to] {
            continue;
        }

        let c = decomposition(to, used, size, res, t);
        res[c] = centroid as i32 + 1;
    }

    used[centroid] = false;

    centroid
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut used = vec![false; n];
    let mut size = vec![0; n];
    let mut res = vec![-1; n];
    decomposition(0, &mut used, &mut size, &mut res, &t);

    println!("{}", res.into_iter().join(" "))
}
