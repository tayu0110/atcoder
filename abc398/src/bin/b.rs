use proconio::*;

fn main() {
    input! {a: [usize; 7]}

    for i in 0u32..1 << 7 {
        if i.count_ones() != 5 {
            continue;
        }

        let mut buf = [0; 14];
        for j in 0..7 {
            if i & (1 << j) != 0 {
                buf[a[j]] += 1;
            }
        }

        if buf.contains(&3) && buf.contains(&2) {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
