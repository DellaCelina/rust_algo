fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let ascii = input.trim().as_bytes()[0];
    println!("{}", ascii);
}
