use std::collections::BTreeSet;

use proconio::input;

fn dfs(now: usize, n: usize, t: &mut Vec<usize>, ck: &mut BTreeSet<(usize, Vec<usize>)>) {
    ck.insert((now, t.clone()));
    if n == 13 {
        return;
    }
    
    let prev = match t.last() {
        Some(prev) => *prev,
        None => 1
    };
    for i in prev..10 {
        t.push(i);
        dfs(now * i, n + 1, t, ck);
        t.pop().unwrap();
    }
}

fn main() {
    input! {n: usize, b: usize};

    if n < b {
        println!("0");
        std::process::exit(0);
    }

    let mut ck = BTreeSet::new();
    dfs(1, 0, &mut vec![], &mut ck);

    let mut res = 0;

    if b.to_string().contains('0') {
        res = 1;
    }

    for (fm, mut v) in ck {
        let mut m = b + fm;
        if m > n {
            continue;
        }
        let mut bad = false;
        while m > 0 {
            let now = m % 10;
            if v.contains(&now) {
                let len = v.len();
                for j in 0..len {
                    if v[j] == now {
                        v.remove(j);
                        break;
                    }
                }
            } else {
                bad = true;
                break;
            }
            m /= 10;
        }

        if v.len() > 0 {
            bad = true;
        }

        if !bad {
            res += 1;
        }

    }

    println!("{}", res);
}