use algebrae::{algebra::Group, modular::Gf32};
use data_strux::disjoint_set::PotentialDsu;
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

    let mut dsu = PotentialDsu::new(n);

    for _ in 0..q {
        let t = parse!(u8);
        let u = parse!(usize);
        let v = parse!(usize);
        if t == 0 {
            let x0 = parse!(u32);
            let x1 = parse!(u32);
            let x2 = parse!(u32);
            let x3 = parse!(u32);
            writeln!(
                stdout,
                "{}",
                dsu.unite(
                    v,
                    u,
                    Gf32Matrix(Gf32::new(x0), Gf32::new(x1), Gf32::new(x2), Gf32::new(x3))
                ) as u8
            )
            .ok();
        } else {
            if let Some(ans) = dsu.potential(v, u) {
                writeln!(stdout, "{} {} {} {}", ans.0, ans.1, ans.2, ans.3).ok();
            } else {
                writeln!(stdout, "-1").ok();
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Gf32Matrix(Gf32<MOD>, Gf32<MOD>, Gf32<MOD>, Gf32<MOD>);

impl Group for Gf32Matrix {
    fn id() -> Self {
        Self(Gf32::new(1), Gf32::new(0), Gf32::new(0), Gf32::new(1))
    }

    fn op(&self, rhs: &Self) -> Self {
        Self(
            self.0 * rhs.0 + self.1 * rhs.2,
            self.0 * rhs.1 + self.1 * rhs.3,
            self.2 * rhs.0 + self.3 * rhs.2,
            self.2 * rhs.1 + self.3 * rhs.3,
        )
    }

    fn inv(&self) -> Self {
        Self(self.3, -self.1, -self.2, self.0)
    }
}
