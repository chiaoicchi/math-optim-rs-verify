use data_strux::segment_tree::{Action, LazySegmentTree, Monoid};
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

    let a: Vec<LenSum> = (0..n).map(|_| LenSum(parse!(u32), 1)).collect();
    let mut lazy_segment_tree = LazySegmentTree::from_vec(a);

    for _ in 0..q {
        let t = parse!(u8);
        if t == 0 {
            let l: usize = parse!(usize);
            let r: usize = parse!(usize);
            let c: u32 = parse!(u32);
            let d: u32 = parse!(u32);
            lazy_segment_tree.range_apply(l..r, Affine(c, d));
        } else {
            let l = parse!(usize);
            let r = parse!(usize);
            let ans = lazy_segment_tree.range_fold(l..r).0;
            writeln!(stdout, "{}", ans).ok();
        }
    }
}

const MOD: u32 = 998_244_353;

#[derive(Clone)]
struct LenSum(u32, u32);

impl Monoid for LenSum {
    fn id() -> Self {
        LenSum(0, 0)
    }

    fn op(&self, other: &Self) -> Self {
        let s = self.0 + other.0;
        let l = self.1 + other.1;
        LenSum(
            if s >= MOD { s - MOD } else { s },
            if l >= MOD { l - MOD } else { l },
        )
    }
}

#[derive(Clone)]
struct Affine(u32, u32);

impl Monoid for Affine {
    fn id() -> Self {
        Affine(1, 0)
    }

    fn op(&self, other: &Self) -> Self {
        let Affine(a0, b0) = self;
        let Affine(a1, b1) = other;
        let a = (*a0 as u64 * *a1 as u64 % MOD as u64) as u32;
        let b = (*a0 as u64 * *b1 as u64 % MOD as u64) as u32 + b0;
        Affine(a, if b >= MOD { b - MOD } else { b })
    }
}

impl Action<LenSum> for Affine {
    fn act(&self, s: &LenSum) -> LenSum {
        let sum = (self.0 as u64 * s.0 as u64 % MOD as u64) as u32
            + (self.1 as u64 * s.1 as u64 % MOD as u64) as u32;
        LenSum(if sum >= MOD { sum - MOD } else { sum }, s.1)
    }
}
