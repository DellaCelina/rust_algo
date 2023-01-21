fn get_in() -> Vec<u32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .map(str::parse)
        .flatten()
        .collect()
}

fn main() {
    let &[_, x, ..] = get_in().as_slice() else { panic!() };
    get_in()
        .into_iter()
        .filter(|&v| v < x)
        .for_each(|x| print!("{} ", x));
}
