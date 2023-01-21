fn main() {
    use std::cmp::Ordering;

    let nums: Vec<_> = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    };

    match nums[1].cmp(&nums[0]) {
        Ordering::Greater => println!("<"),
        Ordering::Less => println!(">"),
        Ordering::Equal => println!("=="),
    }
}
