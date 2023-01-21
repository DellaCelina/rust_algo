fn main() {
    const DIFF: u32 = 2541 - 1998;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let out = input.trim().parse::<u32>().unwrap() - DIFF;
    println!("{}", out);
}
