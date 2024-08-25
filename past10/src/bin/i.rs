use proconio::*;

fn main() {
    input! {n: usize, mut p: [(i32, i32); n], mut q: [(i32, i32); n]}
    p.sort_unstable();
    q.sort_unstable();

    if p == q {
        println!("Yes");
        return;
    }

    {
        let mut q = q.clone();
        q.iter_mut().for_each(|v| v.0 = -v.0);

        let pmin = p.iter().map(|v| v.0).min().unwrap();
        let qmin = q.iter().map(|v| v.0).min().unwrap();

        q.iter_mut().for_each(|v| v.0 += pmin - qmin);
        q.sort_unstable();

        if p == q {
            println!("Yes");
            return;
        }
    }

    {
        q.iter_mut().for_each(|v| v.1 = -v.1);

        let pmin = p.iter().map(|v| v.1).min().unwrap();
        let qmin = q.iter().map(|v| v.1).min().unwrap();
        q.iter_mut().for_each(|v| v.1 += pmin - qmin);
        q.sort_unstable();

        if p == q {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
