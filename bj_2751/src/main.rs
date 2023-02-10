use std::fmt::Write;

fn main() {
    let n = get_in();
    let mut v: Vec<_> = (0..n).map(|_| get_in()).collect();
    v.sort();

    let mut ret = String::new();
    v.iter().for_each(|x| write!(ret, "{x}\n").unwrap());
    println!("{ret}");
}

fn get_in() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
