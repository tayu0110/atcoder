fn main() {
    let mut buf = vec![1, 1];
    const INF: usize = 4_000_000;

    let mut sum = 0;
    loop {
        let len = buf.len();
        let new = buf[len-1] + buf[len-2];
        if new > INF {
            break;
        }
        if new % 2 == 0 {
            sum += new;
        }
        buf.push(new);
    }

    println!("{}", sum);
}