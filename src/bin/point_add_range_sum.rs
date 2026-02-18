use data_strux::fenwick_tree::{FenwickTree, Group, Monoid};
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

    let a: Vec<Add> = (0..n).map(|_| Add(parse!(u64))).collect();
    let mut fenwick_tree = FenwickTree::<Add>::from_vec(a);

    for _ in 0..q {
        let t = parse!(u8);
        if t == 0 {
            let p = parse!(usize);
            let x = parse!(u64);
            fenwick_tree.operate(p, Add(x));
        } else {
            let l = parse!(usize);
            let r = parse!(usize);
            writeln!(stdout, "{}", fenwick_tree.range_fold(l..r).0).ok();
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
