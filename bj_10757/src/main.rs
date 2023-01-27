use core::fmt;
use std::fmt::Display;
use std::ops::Add;

struct BigInt {
    v: Vec<u8>,
}

impl BigInt {
    fn new(v: Vec<u8>) -> BigInt {
        BigInt { v }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, op: Self) -> Self {
        let get_chain_iter = |n| [0].into_iter().cycle().take(n);

        let s_len = self.v.len();
        let op_len = op.v.len();
        let len = usize::max(s_len, op_len);

        let s_iter = self.v.into_iter().rev().chain(get_chain_iter(len - s_len));
        let op_iter = op.v.into_iter().rev().chain(get_chain_iter(len - op_len));

        let out = s_iter
            .zip(op_iter)
            .map(|(a, b)| a + b)
            .chain([0])
            .scan(0, |carry, v| {
                let ret = v + *carry;
                *carry = ret / 10;
                Some(ret % 10)
            })
            .collect::<Vec<_>>();

        let out: (Vec<_>, Vec<_>) = out
            .into_iter()
            .rev()
            .enumerate()
            .skip_while(|&(i, v)| i == 0 && v == 0)
            .unzip();

        BigInt { v: out.1 }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.v.iter().try_for_each(|v| write!(f, "{v}"))
    }
}

fn main() {
    let input = get_in();
    let out = input.into_iter().reduce(|a, b| a + b).unwrap();
    println!("{}", out);
}

fn get_in() -> Vec<BigInt> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace()
        .map(|s| {
            s.as_bytes()
                .into_iter()
                .map(|&v| v - b'0')
                .collect::<Vec<_>>()
        })
        .map(|v| BigInt::new(v))
        .collect()
}
