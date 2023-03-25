fn main() {
    proconio::input! {n: usize, a: [usize; n]}
    let map = a.into_iter().map(|v| (v, 1)).collect::<std::collections::HashMap<_, _>>();
    println!("{}", (1..=n).scan(0, |s, i| { *s += 2 - *map.get(&i).unwrap_or(&0); Some(*s) }).take_while(|v| *v <= n).count());
}