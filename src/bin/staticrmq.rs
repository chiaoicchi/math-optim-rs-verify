use algebrae::algebra::Band;
use data_strux::sparse_table::SparseTable;
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

    let a: Vec<Min> = (0..n).map(|_| Min(parse!(u32))).collect();
    let sparse_table = SparseTable::<Min>::from_vec(a);

    for _ in 0..q {
        let l = parse!(usize);
        let r = parse!(usize);
        writeln!(stdout, "{}", sparse_table.range_fold(l..r).0).ok();
    }
}

#[derive(Clone, Copy)]
struct Min(u32);

impl Band for Min {
    fn op(&self, other: &Self) -> Self {
        Min(self.0.min(other.0))
    }
}
