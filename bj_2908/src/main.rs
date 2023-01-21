fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let ret = input
        .split_ascii_whitespace()
        .map(|s| {
            s.to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", ret);
}
