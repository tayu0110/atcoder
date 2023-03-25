fn main() {
    let mut map = std::collections::BTreeMap::new();
    let mut res = std::collections::HashSet::new();
    let mut set = std::collections::HashSet::<usize>::new();
    for i in 1..100 {
        let mut tmp = std::collections::HashSet::new();
        for j in 1..=1000000usize {
            if !set.contains(&j) {
                if !res.contains(&j) {
                    map.insert(j, i);
                    res.insert(j);
                }
                tmp.insert(j);
                for k in &set {
                    tmp.insert(*k ^ j);
                }
                break;
            }
        }

        std::mem::swap(&mut set, &mut tmp);
    }

    println!("{:?}", map);
}
