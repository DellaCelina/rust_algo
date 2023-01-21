fn get_in() -> Vec<u32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let total = get_in()[0];
    let n = get_in()[0];
    let ret = (0..n).fold(0, |acc, _| acc + get_in().into_iter().product::<u32>());
    if ret == total {
        println!("Yes");
    } else {
        println!("No");
    }
}
