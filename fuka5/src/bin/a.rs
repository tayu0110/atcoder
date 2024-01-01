use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().lock().read_to_string(&mut s).ok();
    let mut s = s.split_ascii_whitespace();
    while let Some(n) = s.next().map(|s| s.parse::<usize>().unwrap()) {
        let k: usize = s.next().unwrap().parse().unwrap();
        if n == 0 && k == 0 {
            break;
        }
        let mut x: Vec<usize> = s.by_ref().take(n).map(|s| s.parse().unwrap()).collect();
        x.sort_unstable();
        println!("{}", x[..k].iter().sum::<usize>())
    }
}
