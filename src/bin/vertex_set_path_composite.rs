use algebrae::{algebra::Monoid, num_theory::Gf};
use data_strux::segment_tree::SegmentTree;
use graphia::{csr::Csr, tree::Hpd};
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
    let ab: Vec<Affine> = (0..n)
        .map(|_| Affine(Gf::new(parse!(u32)), Gf::new(parse!(u32))))
        .collect();
    let uv: Vec<(usize, usize)> = (0..n - 1).map(|_| (parse!(usize), parse!(usize))).collect();

    let csr = Csr::from_undirected_unweighted(n, &uv);
    let hpd = Hpd::from_csr(0, &csr);
    let sa: Vec<Affine> = (0..n).map(|i| ab[hpd.vertex(i)]).collect();
    let sa_rev: Vec<Affine> = (0..n).rev().map(|i| ab[hpd.vertex(i)]).collect();
    let mut seg = SegmentTree::from_vec(sa);
    let mut seg_rev = SegmentTree::from_vec(sa_rev);

    for _ in 0..q {
        let t = parse!(u8);
        if t == 0 {
            let p = parse!(usize);
            let c = Gf::new(parse!(u32));
            let d = Gf::new(parse!(u32));
            seg.set(hpd.pos(p), Affine(c, d));
            seg_rev.set(n - 1 - hpd.pos(p), Affine(c, d));
        } else {
            let u = parse!(usize);
            let v = parse!(usize);
            let x = Gf::new(parse!(u32));
            let mut ans = Affine::id();
            hpd.path_vertex(u, v, |l, r, forward| {
                if forward {
                    ans = ans.op(&seg.range_fold(l..r));
                } else {
                    ans = ans.op(&seg_rev.range_fold(n - r..n - l));
                }
            });
            writeln!(stdout, "{}", ans.0 * x + ans.1).ok();
        }
    }
}

#[derive(Clone, Copy)]
struct Affine(Gf<MOD>, Gf<MOD>);

impl Monoid for Affine {
    fn id() -> Self {
        Affine(Gf::new(1), Gf::new(0))
    }
    fn op(&self, rhs: &Self) -> Self {
        let Affine(a0, b0) = self;
        let Affine(a1, b1) = rhs;
        Affine(a0 * a1, a1 * b0 + b1)
    }
}
