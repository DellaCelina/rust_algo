fn get_in() -> Vec<u32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect()
}

fn main() {
    use std::collections::BTreeSet;
    let ret = (0..10)
        .map(|_| get_in()[0] % 42)
        .collect::<BTreeSet<_>>()
        .iter()
        .count();
    println!("{}", ret);
}
