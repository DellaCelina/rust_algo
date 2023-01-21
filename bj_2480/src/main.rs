fn main() {
    let nums: Vec<_> = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect()
    };

    match nums.as_slice() {
        &[a, b, c] if (a == b) && (b == c) => println!("{}", 10_000 + a * 1000),
        &[a, b, _] if a == b => println!("{}", 1_000 + a * 100),
        &[_, b, c] if b == c => println!("{}", 1_000 + b * 100),
        &[a, _, c] if a == c => println!("{}", 1_000 + a * 100),
        _ => println!("{}", nums.iter().max().unwrap() * 100),
    }
}
