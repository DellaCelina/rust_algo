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
    get_in();
    let nums = get_in();
    let golden = get_in()[0];

    let ret = nums.iter().filter(|&&x| x == golden).count();
    println!("{}", ret);
}
