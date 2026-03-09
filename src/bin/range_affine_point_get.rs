use algebrae::{
    algebra::{Action, Monoid},
    num_theory::Gf,
};
use data_strux::segment_tree::DualSegmentTree;
use std::io::{BufWriter, Read, Write, stdin, stdout};

const MOD: u32 = 998_244_353;

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

    let a: Vec<Gf<MOD>> = (0..n).map(|_| Gf::new(parse!(u32))).collect();
    let mut dual_segment_tree = DualSegmentTree::<Gf<MOD>, Affine>::from_vec(a);

    for _ in 0..q {
        let t = parse!(u8);
        if t == 0 {
            let l: usize = parse!(usize);
            let r: usize = parse!(usize);
            let b: u32 = parse!(u32);
            let c: u32 = parse!(u32);
            dual_segment_tree.range_apply(l..r, Affine(Gf::new(b), Gf::new(c)));
        } else {
            let i = parse!(usize);
            writeln!(stdout, "{}", dual_segment_tree.get(i)).ok();
        }
    }
}

#[derive(Clone)]
struct Affine(Gf<MOD>, Gf<MOD>);

impl Monoid for Affine {
    fn id() -> Self {
        Affine(Gf::new(1), Gf::new(0))
    }

    fn op(&self, other: &Self) -> Self {
        let Affine(a0, b0) = self;
        let Affine(a1, b1) = other;
        Affine(a0 * a1, a0 * b1 + b0)
    }
}

impl Action<Gf<MOD>> for Affine {
    fn act(&self, s: &Gf<MOD>) -> Gf<MOD> {
        self.0 * s + self.1
    }
}
