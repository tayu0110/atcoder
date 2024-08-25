use proconio::input;

fn main() {
    input! {n: usize, m: i64, a: [i64; n]}

    let mut map = std::collections::BTreeMap::new();
    for (i, a) in a.into_iter().enumerate() {
        let i = i as i64+ 1;
        let mut start = 1;
        if a < 0 {
            start = (a.abs() + i - 1) / i;
        }

        for j in (start..=m).take_while(|j| a + i * *j <= n as i64) {
            map.entry(j).or_insert(vec![]).push(a + j * i);
        }
    }

    for i in 1..=m {
        if let Some(v) = map.get(&i) {
            let set = v.iter().copied().collect::<std::collections::HashSet<_>>();
            println!("{}", (0..=n as i64).take_while(|v| set.contains(v)).count());
        } else {
            println!("0");
        }
    }
}