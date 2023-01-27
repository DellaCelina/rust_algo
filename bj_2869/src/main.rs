fn main() {
    let nums = get_in();

    let [a, b, c, ..] = nums.as_slice() else { panic!() };

    let ret = ((c - b) as f64 / (a - b) as f64).ceil();
    println!("{}", ret);
}

fn get_in() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace()
        .flat_map(str::parse::<u32>)
        .collect()
}
