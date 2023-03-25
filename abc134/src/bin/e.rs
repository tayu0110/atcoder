use proconio::input;

fn main() {
    input! {n: usize, mut a: [i64; n]}
    a.iter_mut().for_each(|v| *v = (*v + 1) * 100000);

    let mut map = std::collections::HashMap::new();
    for na in a.iter_mut() {
        let entry = map.entry(*na).or_insert(0);
        *na -= *entry;
        *entry += 1;
    }

    let mut a = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<Vec<(_, _)>>();
    a.sort();

    let mut set = std::collections::BTreeSet::new();
    for (_, i) in a {
        if let Some(&j) = set.range(0..i).next_back() {
            set.remove(&j);
        }
        set.insert(i);

        // eprintln!("set: {:?}", set);
    }

    println!("{}", set.len());
}
