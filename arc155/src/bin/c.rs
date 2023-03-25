use proconio::input;

fn check(a: &[usize], b: &[usize]) -> bool {
    let mut map = std::collections::HashMap::new();
    for &a in a {
        *map.entry(a).or_insert(0) += 1;
    }
    for &b in b {
        if !map.contains_key(&b) {
            return false;
        }
        let entry = map.entry(b).or_insert(0);
        *entry -= 1;

        if *entry == 0 {
            map.remove(&b);
        }
    }

    true
}

fn solve(a: &[usize], b: &[usize]) -> bool {
    if a == b {
        return true;
    }

    if !check(&a, &b) {
        return false;
    }

    if a.iter().all(|&a| a % 2 == 1) {
        return false;
    }

    if a.iter().all(|&a| a % 2 == 0) {
        return true;
    }

    if a.windows(3)
        .map(|v| {
            let mut w = [0; 2];
            for v in v {
                w[v % 2] += 1;
            }
            w
        })
        .filter(|v| v != &[3, 0])
        .all(|v| v == [2, 1] || v == [0, 3])
    {
        if a.iter()
            .zip(b.iter())
            .any(|(a, b)| (a % 2 == 1 || b % 2 == 1) && a != b)
        {
            return false;
        }

        let mut na = vec![];
        let mut nb = vec![];
        for (&a, &b) in a.iter().zip(b.iter()) {
            if a % 2 == 1 {
                if na.len() >= 3 {
                    if !check(&na, &nb) {
                        return false;
                    }
                } else {
                    if na != nb {
                        return false;
                    }
                }

                na = vec![];
                nb = vec![];
            } else {
                na.push(a);
                nb.push(b);
            }
        }

        if na.len() >= 3 {
            if !check(&na, &nb) {
                return false;
            }
        } else {
            if na != nb {
                return false;
            }
        }
    }

    if a.iter().filter(|&a| a % 2 == 0).count() == 2 {
        let na = a.iter().filter(|&a| a % 2 == 0).collect::<Vec<_>>();
        let nb = b.iter().filter(|&b| b % 2 == 0).collect::<Vec<_>>();

        if na != nb {
            return false;
        }
    }

    true
}

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}

    if solve(&a, &b) && solve(&b, &a) {
        println!("Yes")
    } else {
        println!("No")
    }
}
