fn get_in() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let n: usize = get_in().parse().unwrap();
    let num = get_in();

    let ret = num
        .as_bytes()
        .iter()
        .take(n)
        .map(|&x| (x - '0' as u8) as u32)
        .sum::<u32>();
    println!("{}", ret);
}
