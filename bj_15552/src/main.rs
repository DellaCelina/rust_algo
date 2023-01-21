use std::fmt::Write;
use std::io::stdin;
fn main() {
    let num: u32 = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    let ret: String = {
        let mut ret = String::new();
        let adds = (0..num).map(|_| {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input
                .split_ascii_whitespace()
                .flat_map(str::parse::<i32>)
                .reduce(|a, b| a + b)
                .unwrap()
        });
        adds.for_each(|x| writeln!(ret, "{x}").unwrap());
        ret
    };
    println!("{}", ret);
}
