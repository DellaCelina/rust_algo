fn get_in() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>)
        .collect()
}

fn main() {
    let nums: Vec<_> = (0..9).map(|_| get_in()[0]).collect();
    let &max = nums.iter().max().unwrap();
    let pos = nums.iter().position(|&x| x == max).unwrap();
    println!("{}", max);
    println!("{}", pos + 1);
}
