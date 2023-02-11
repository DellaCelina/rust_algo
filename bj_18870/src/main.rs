use std::collections::HashMap;

fn main() {
    io::get_v::<u32>().iter().for_each(|_| {
        io::get_vec::<i32>()
            .iter()
            .for_each(|v| {
                let mut hash_v = v.clone();
                hash_v.sort();
                hash_v.dedup();
                let hash: HashMap<_, _> = hash_v.into_iter()
                    .zip(0..)
                    .collect();

                let ret: Vec<_> = v.iter().flat_map(|x| hash.get(x)).collect();
                io::print_vec_as_line(&ret);
            });
    });
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

    pub fn print_vec_as_line<T: fmt::Display>(arr: &Vec<T>) {
        let mut buf_out = io::BufWriter::new(io::stdout().lock());
        arr.iter()
            .for_each(|v| write!(buf_out, "{} ", *v).unwrap());
    }
}