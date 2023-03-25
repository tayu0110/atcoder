use itertools::Itertools;

fn main() {
    let (l, r) = (100, 1000);
    let mut res = 0;

    for i in l..r {
        for j in i..r {
            let m = i * j;
            let m = m.to_string().chars().collect_vec();
            let mut n = m.clone();
            n.reverse();
            let m = m.into_iter().collect::<String>();
            let n = n.into_iter().collect::<String>();

            if m == n {
                res = std::cmp::max(res, i*j);
            }
        }
    }

    println!("{}", res);
}