use std::fmt::Write;

fn main() {
    let n = get_in();
    let mut arr: Vec<_> = (0..n).map(|_| get_in()).collect();
    arr.sort();

    let mut ret = String::new();
    arr.into_iter().for_each(|v| write!(ret, "{v}\n").unwrap());
    println!("{ret}");
}

fn get_in() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
