fn main() {
    let keys = ["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim();
    let mut cnt = 0;
    while !input.is_empty() {
        input = keys
            .iter()
            .flat_map(|&k| input.strip_prefix(k))
            .nth(0)
            .unwrap_or(&input[1..]);
        cnt += 1;
    }
    println!("{}", cnt);
}
