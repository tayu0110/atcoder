use proconio::input;

fn main() {
    input! {n: usize, l: usize, a: [usize; n]}

    let mut nt = std::collections::BinaryHeap::new();
    nt.push(l);

    for a in a {
        if let Some(l) = nt.pop() {
            if l < a {
                println!("No");
                return;
            }

            if l > a {
                nt.push(1);
                let l = l - a - 1;
                if l > 0 {
                    nt.push(l);
                }
            }
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
