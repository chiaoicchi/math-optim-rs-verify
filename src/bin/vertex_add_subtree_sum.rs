use data_strux::fenwick_tree::{FenwickTree, Group, Monoid};
use graphia::{csr::Csr, tree::EulerTour};
use std::io::{BufWriter, Read, Write, stdin, stdout};

fn main() {
    let mut input = Vec::new();
    stdin().lock().read_to_end(&mut input).unwrap();
    let mut iter = input.split(|&b| b <= b' ').filter(|s| !s.is_empty());
    let mut stdout = BufWriter::new(stdout().lock());

    macro_rules! parse {
        ($t:ty) => {{
            let s = iter.next().unwrap();
            let mut x: $t = 0;
            for &b in s {
                x = x * 10 + (b - b'0') as $t;
            }
            x
        }};
    }

    let n = parse!(usize);
    let q = parse!(u32);
    let a: Vec<u64> = (0..n).map(|_| parse!(u64)).collect();
    let p: Vec<(usize, usize)> = (1..n).map(|i| (parse!(usize), i)).collect();

    let csr = Csr::from_directed_unweighted(n, &p);
    let euler_tour = EulerTour::from_csr(0, &csr);
    let order = euler_tour.order();
    let mut v = Vec::with_capacity(n);
    for &i in order.iter() {
        v.push(Add(a[i]));
    }
    let mut fenwick_tree = FenwickTree::<Add>::from_vec(v);

    for _ in 0..q {
        if parse!(u8) == 0 {
            let u = parse!(usize);
            let x = parse!(u64);
            fenwick_tree.operate(euler_tour.tin(u), Add(x));
        } else {
            let u = parse!(usize);
            writeln!(
                stdout,
                "{}",
                fenwick_tree.range_fold(euler_tour.subtree(u)).0
            )
            .ok();
        }
    }
}

#[derive(Clone, Copy)]
struct Add(u64);

impl Monoid for Add {
    fn id() -> Self {
        Add(0)
    }

    fn op(&self, other: &Self) -> Self {
        Add(self.0.wrapping_add(other.0))
    }
}

impl Group for Add {
    fn inv(&self) -> Self {
        Add(self.0.wrapping_neg())
    }
}
