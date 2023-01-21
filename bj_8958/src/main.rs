use std::io::stdin;

fn main() {
    let n: u32 = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    (0..n)
        .map(|_| {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input.trim().split(|c| c == 'X').fold(0, |acc, s| {
                let len = s.len();
                let ret = (len * (len + 1)) / 2;
                acc + ret
            })
        })
        .for_each(|x| println!("{}", x));
}
