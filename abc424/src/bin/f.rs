use ds::{LinkCutTree, MapMonoid};
use itertools::Itertools;

struct T;
impl MapMonoid for T {
    type M = ();
    type Act = ();

    fn e() -> Self::M {}
    fn op(_: &Self::M, _: &Self::M) -> Self::M {}
    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(_: &Self::M, _: &Self::Act) -> Self::M {}
}

fn main() {
    cpio::scan! {n: usize, q: usize, p: [(u32, u32); q]}

    let mut ett = LinkCutTree::<T>::new(n * 2);
    for i in 0..n {
        ett.link(i * 2, i * 2 + 1).unwrap();
        if i > 0 {
            ett.link((i - 1) * 2 + 1, i * 2).unwrap();
        }
    }

    let mut ret = Vec::with_capacity(q);
    for (a, b) in p {
        let (a, b) = (a as usize - 1, b as usize - 1);

        let are_connected = ett.is_connected(a * 2, b * 2);
        ret.push(are_connected);

        if are_connected {
            ett.cut(a * 2 + 1);
            ett.cut(b * 2 + 1);
            ett.link(a * 2, b * 2 + 1).unwrap();
            // ett.link(a * 2 + 1, b * 2).ok();
        }
    }

    cpio::putln!(ret
        .iter()
        .map(|ret| if *ret { "Yes" } else { "No" })
        .join("\n"));
}
