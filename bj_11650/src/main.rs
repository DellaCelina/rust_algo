use std::fmt::{Display, Formatter};

struct Point<'a> {
    v: &'a Vec<i32>,
}

impl<'a> Display for Point<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.v[0], self.v[1])
    }
}

fn main() {
    let n: u32 = io::get_v().unwrap();

    let mut arr: Vec<_> = (0..n)
        .into_iter()
        .flat_map(|_| io::get_vec::<i32>())
        .collect();
    arr.sort_by(|a, b| {
        use std::cmp::Ordering;
        match a[0].cmp(&b[0]) {
            Ordering::Equal => a[1].cmp(&b[1]),
            v @ _ => v,
        }
    });

    let ret: Vec<_> = arr.iter().map(|v| Point { v }).collect();
    io::print_vec(&ret);
}

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
