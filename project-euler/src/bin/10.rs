fn main() {
    const MAX: usize = 2_000_000;
    let mut p = vec![std::usize::MAX; MAX];
    p[0] = 0;
    p[1] = 0;

    for now in 2..MAX {
        if p[now] == std::usize::MAX {
            for i in (2..MAX).take_while(|v| now * *v < MAX) {
                p[now * i] = now;
            }
        }
    }

    let res = (0..MAX).filter(|v| p[*v] == std::usize::MAX).sum::<usize>();
    println!("{}", res);
}