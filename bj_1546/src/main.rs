fn get_in() -> Vec<u32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect()
}

fn main() {
    let n = get_in()[0];
    let nums = get_in();
    let &max = nums.iter().max().unwrap();
    let ret = nums
        .iter()
        .map(|&x| x as f64 / max as f64 * 100_f64)
        .sum::<f64>()
        / n as f64;
    println!("{}", ret);
}
