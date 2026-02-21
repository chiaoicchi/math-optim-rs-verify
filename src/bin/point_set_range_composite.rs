use data_strux::segment_tree::{Monoid, SegmentTree};
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

    let a: Vec<Affine> = (0..n).map(|_| Affine((parse!(u32), parse!(u32)))).collect();
    let mut segment_tree = SegmentTree::from_vec(a);

    for _ in 0..q {
        let t = parse!(u8);
        if t == 0 {
            let p = parse!(usize);
            let c = parse!(u32);
            let d = parse!(u32);
            segment_tree.set(p, Affine((c, d)));
        } else {
            let l = parse!(usize);
            let r = parse!(usize);
            let x = parse!(u64);
            let (c, d) = segment_tree.range_fold(l..r).0;
            let ans = (c as u64 * x % MOD as u64) as u32 + d;
            writeln!(stdout, "{}", if ans >= MOD { ans - MOD } else { ans }).ok();
        }
    }
}

const MOD: u32 = 998_244_353;

#[derive(Clone, Copy)]
struct Affine((u32, u32));

impl Monoid for Affine {
    fn id() -> Self {
        Affine((1, 0))
    }

    fn op(&self, other: &Self) -> Self {
        let a = (self.0.0 as u64 * other.0.0 as u64 % MOD as u64) as u32;
        let b = (other.0.0 as u64 * self.0.1 as u64 % MOD as u64) as u32 + other.0.1;
        Affine((a, if b >= MOD { b - MOD } else { b }))
    }
}
