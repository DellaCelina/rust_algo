fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut arr: Vec<u8> = input.as_bytes().into_iter().map(|&v| v).collect();
    arr.sort_by(|a, b| b.cmp(a));

    let mut ret = String::new();
    arr.iter().for_each(|&v| ret.push(v as char));

    println!("{ret}");
}
