#[allow(dead_code)]
mod io {
    use std::fmt;
    use std::io::{self, Write};
    use std::num;
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum Error {
        IO(io::Error),
        ParseInt(num::ParseIntError),
        ParseFloat(num::ParseFloatError),
    }

    impl From<io::Error> for Error {
        fn from(e: io::Error) -> Self {
            Error::IO(e)
        }
    }

    impl From<num::ParseIntError> for Error {
        fn from(e: num::ParseIntError) -> Self {
            Error::ParseInt(e)
        }
    }

    impl From<num::ParseFloatError> for Error {
        fn from(e: num::ParseFloatError) -> Self {
            Error::ParseFloat(e)
        }
    }

    pub fn get_v<T>() -> Result<T, Error>
    where
        T: FromStr,
        Error: From<<T as FromStr>::Err>,
    {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        Ok(s.trim().parse::<T>()?)
    }

    pub fn get_vec<T>() -> Result<Vec<T>, Error>
    where
        T: FromStr,
        Error: From<<T as FromStr>::Err>,
    {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        s.split_ascii_whitespace()
            .map(str::parse)
            .map(|r: Result<T, _>| r.map_err(|e: _| e.into()))
            .collect()
    }

    pub fn print_vec<T: fmt::Display>(arr: &Vec<T>) {
        let mut buf_out = io::BufWriter::new(io::stdout().lock());
        arr.iter()
            .for_each(|v| write!(buf_out, "{}\n", *v).unwrap());
    }
}

use std::collections::HashMap;

struct Statistics {
    sum: i32,
    mode: HashMap<i32, u32>,
}

impl Statistics {
    fn new() -> Statistics {
        Statistics {
            sum: 0,
            mode: HashMap::new(),
        }
    }

    fn apply(&mut self, v: i32) {
        self.sum += v;
        let mode = self.mode.entry(v).or_insert(0);
        *mode += 1;
    }

    fn get_mode(&self) -> i32 {
        use std::cmp::Ordering;

        let mut mode_arr: Vec<_> = self.mode.iter().collect();
        mode_arr.sort_by(|a, b| match b.1.cmp(a.1) {
            Ordering::Equal => a.0.cmp(b.0),
            v @ _ => v,
        });

        let ret = (0..2)
            .flat_map(|i| mode_arr.get(i))
            .reduce(|a, b| if a.1 > b.1 { a } else { b })
            .unwrap()
            .0;

        *ret
    }
}

fn main() {
    let n: i32 = io::get_v().unwrap();
    let mut arr: Vec<i32> = (0..n).map(|_| io::get_v().unwrap()).collect();
    arr.sort();

    let mut stat = Statistics::new();
    arr.iter().fold(&mut stat, |st, v| {
        st.apply(*v);
        st
    });

    let avg = ((stat.sum as f32) / (n as f32)).round() as i32;
    let &mid = arr.get((n / 2) as usize).unwrap();
    let mode = stat.get_mode();
    let range = arr
        .last()
        .and_then(|&l| arr.first().map(|&f| l - f))
        .unwrap();

    println!("{avg}");
    println!("{mid}");
    println!("{mode}");
    println!("{range}");
}
