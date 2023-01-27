fn main() {
    let mut n = get_in();

    (2..=n).for_each(|v| {
        while n % v == 0 {
            n /= v;
            println!("{v}");
        }
    });
}

fn get_in() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
