use proconio::input;

fn solve(n: usize, a: &[usize]) -> usize {
    let mut res = 0;
    let mut pos = vec![];
    let mut cum = vec![0];

    for &a in a {
        if pos.is_empty() {
            pos.push(a);
            cum.push(a + 1);
            continue;
        }

        let (mut l, mut r) = (-1, pos.len() as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if pos[m as usize] < n - a {
                l = m;
            } else {
                r = m;
            }
        }

        if l >= 0 {
            res += (pos.len() - 1 - l as usize) * (n - a);
            res += cum[r as usize];
        } else {
            res += pos.len() * (n - a);
        }

        pos.push(a);
        cum.push(*cum.last().unwrap() + a + 1);
    }

    res
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut t = vec![vec![]; n];
    for (i, a) in a.into_iter().enumerate() {
        t[a - 1].push(i);
    }

    let mut res = {
        let mut sum = 0;
        for i in 1..=n {
            let k = n + 1 - i;
            sum += i / 2 * k;
        }
        sum
    };

    // eprintln!("res: {}", res);

    for i in 0..n {
        res -= solve(n, &t[i]);
    }

    println!("{}", res);
}
