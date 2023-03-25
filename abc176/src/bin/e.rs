use proconio::input;

fn main() {
    input! {_h: usize, _w: usize, m: usize, p: [(usize, usize); m]}

    let mut row = std::collections::HashMap::new();
    let mut column = std::collections::HashMap::new();
    for (h, w) in p {
        *row.entry(h).or_insert(0) += 1;
        column.entry(w).or_insert(vec![]).push(h);
    }

    let mut max = 0;
    let mut set = std::collections::HashSet::new();
    for (_, v) in column {
        if v.len() > max {
            max = v.len();
            set = v.into_iter().collect();
        } else if v.len() == max {
            let tmp = v.into_iter().collect();
            set = set.intersection(&tmp).cloned().collect();
        }
    }

    let mut res = 0;
    for (k, v) in row {
        if set.contains(&k) {
            res = std::cmp::max(res, v + max - 1);
        } else {
            res = std::cmp::max(res, v + max);
        }
    }

    println!("{}", res);
}
