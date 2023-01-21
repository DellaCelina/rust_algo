fn get_in() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .map(str::parse)
        .flatten()
        .collect()
}

fn main() {
    let _n = get_in()[0];
    let nums = get_in();
    println!(
        "{} {}",
        nums.iter().min().unwrap(),
        nums.iter().max().unwrap()
    );
}
