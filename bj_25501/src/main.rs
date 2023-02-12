fn main() {
    io::get_v().iter().for_each(|&n| {
        let arr: Vec<_> = (0..n)
            .flat_map(|_| {
                io::get_str().map(|s| {
                    let rec_max_n = ((s.len() as f32) / (2.) + 0.1).ceil() as usize;
                    let v: Vec<_> = s.as_bytes().into_iter().map(|&v| v).collect();
                    let cnt = v
                        .iter()
                        .zip(v.iter().rev())
                        .take(rec_max_n)
                        .take_while(|(&a, &b)| a == b)
                        .count();

                    if cnt == rec_max_n {
                        format!("1 {cnt}")
                    } else {
                        format!("0 {}", cnt + 1)
                    }
                })
            })
            .collect();
        io::print_vec(&arr);
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

    pub fn get_str() -> Result<String, Error> {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        Ok(String::from(s.trim()))
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
        arr.iter().for_each(|v| write!(buf_out, "{} ", *v).unwrap());
    }
}
