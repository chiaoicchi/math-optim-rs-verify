use algebrae::algebra::AbelianGroup;
use data_strux::fenwick_tree::FenwickTree;
use graphia::{csr::Csr, tree::Hpd};
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
    let mut a: Vec<Add> = (0..n).map(|_| Add(parse!(u64))).collect();
    let uv: Vec<(usize, usize)> = (0..n - 1).map(|_| (parse!(usize), parse!(usize))).collect();

    let csr = Csr::from_undirected_unweighted(n, &uv);
    let hpd = Hpd::from_csr(0, &csr);
    let mut sa = vec![Add::id(); n + 1];
    for (i, a) in a.iter().enumerate() {
        let sub = hpd.subtree(i);
        sa[sub.start] = sa[sub.start].op(a);
        sa[sub.end] = sa[sub.end].op(&a.inv());
    }
    let mut fenwick = FenwickTree::from_vec(sa);

    for _ in 0..q {
        let t = parse!(u8);
        if t == 0 {
            let p = parse!(usize);
            let x = Add(parse!(u64));
            let sub = hpd.subtree(p);
            a[p] = a[p].op(&x);
            fenwick.operate(sub.start, x);
            fenwick.operate(sub.end, x.inv());
        } else {
            let u = parse!(usize);
            let v = parse!(usize);
            let w = hpd.lca(u, v);
            let pu = fenwick.prefix_fold(hpd.pos(u) + 1);
            let pv = fenwick.prefix_fold(hpd.pos(v) + 1);
            let pw = fenwick.prefix_fold(hpd.pos(w) + 1);
            let ans = pu.op(&pv).op(&pw.inv()).op(&pw.inv()).op(&a[w]);
            writeln!(stdout, "{}", ans.0).ok();
        }
    }
}

#[derive(Clone, Copy)]
struct Add(u64);

impl AbelianGroup for Add {
    fn id() -> Self {
        Add(0)
    }
    fn op(&self, rhs: &Self) -> Self {
        Add(self.0.wrapping_add(rhs.0))
    }
    fn inv(&self) -> Self {
        Add(self.0.wrapping_neg())
    }
}
