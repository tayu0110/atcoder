use proconio::input;

struct MapWrapper<K, V> { map: std::collections::BTreeMap<K, V> }
#[allow(dead_code)]
impl<K, V> MapWrapper<K, V> where K: Ord {
    fn new() -> Self { Self { map: std::collections::BTreeMap::new() } }
    fn len(&self) -> usize { self.map.len() }
    fn contains(&self, key: &K) -> bool { self.map.contains_key(key) }
    fn remove(&mut self, key: &K) { if self.map.contains_key(key) { self.map.remove(key).unwrap(); } }
    fn clear(&mut self) { self.map.clear() }
    fn is_empty(&self) -> bool { self.map.is_empty() }
    fn first(&self) -> Option<(&K, &V)> { if self.is_empty() { None } else { self.map.iter().next() } }
    fn first_mut(&mut self) -> Option<(&K, &mut V)> { if self.is_empty() { None } else { self.map.iter_mut().next() } }
    fn last(&self) -> Option<(&K, &V)> { if self.is_empty() { None } else { self.map.iter().next_back() } }
    fn last_mut(&mut self) -> Option<(&K, &mut V)> { if self.is_empty() { None } else { self.map.iter_mut().next() } }
    fn iter(&self) -> std::collections::btree_map::Iter<'_, K, V> { self.map.iter() }
    fn iter_mut(&mut self) -> std::collections::btree_map::IterMut<'_, K, V> { self.map.iter_mut() }
}
impl<K, V> std::ops::Index<K> for MapWrapper<K, V> where K: Ord {
    type Output = V;
    fn index(&self, index: K) -> &Self::Output { &self.map[&index] }
}
impl<K, V> std::ops::IndexMut<K> for MapWrapper<K, V> where K: Ord, V: Default {
    fn index_mut(&mut self, index: K) -> &mut Self::Output { self.map.entry(index).or_default() }
}

fn main() {
    input! {n: usize, k: usize, a: [usize; n]};

    let mut mp = MapWrapper::<(usize, usize), usize>::new();
    for (i, v) in a.iter().enumerate() {
        mp[(*v, i % k)] += 1;
    }

    let mut a = a.clone();
    a.sort();

    for (i, v) in a.iter().enumerate() {
        if mp.contains(&(*v, i % k)) {
            mp[(*v, i % k)] -= 1;
            if mp[(*v, i % k)] == 0 {
                mp.remove(&(*v, i % k));
            }
        } else {
            println!("No");
            std::process::exit(0);
        }
    }

    println!("Yes");
}